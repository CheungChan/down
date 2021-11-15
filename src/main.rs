extern crate clap;
extern crate reqwest;
extern crate tempdir;
use clap::AppSettings::ArgRequiredElseHelp;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;
use clap::{App, Arg};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use tar::Archive;

#[tokio::main]
async fn main() {
    let app = App::new("down")
        .setting(ArgRequiredElseHelp)
        .version("1.0")
        .author("阿章")
        .about("下载工具")
        .arg(
            Arg::with_name("url")
                .value_name("url")
                .index(1)
                .required(true)
                .help("指定下载的url")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .help("下载保存的目录")
                .takes_value(true),
        )
        .arg(Arg::with_name("tgz").long("tgz").help("是否按照tar.gz解压"));
    let matches = app.get_matches();
    let url = matches.value_of("url").unwrap();
    let to = matches.value_of("to").unwrap_or_else(|| "");
    let to = if to == "" {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    } else {
        to.to_string()
    };
    let filename = match download(url, &to).await {
        Ok(p) => p,
        Err(e) => panic!("下载失败：{}", e),
    };
    if matches.is_present("tgz") {
        if let Err(e) = unpack_file(&filename, &to) {
            eprint!("解压失败：{}", e);
        }
    }
}

pub async fn download(url: &str, to: &str) -> Result<String, anyhow::Error> {
    let url_last = url.split("/").last().unwrap();

    let filename = Path::new(to).join(url_last).to_str().unwrap().to_string();
    let path = Path::new(&filename);
    println!("下载{} 到 {:?}", url, filename);

    let client = Client::new();
    let total_size = {
        let resp = client.head(url).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ));
        }
    };
    let client = Client::new();
    let mut request = client.get(url);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    if path.exists() {
        let size = path.metadata()?.len().saturating_sub(1);
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }
    let mut source = request.send().await?;
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }
    println!("下载完成");
    Ok(filename)
}
pub fn unpack_file(path: &str, to: &str) -> Result<()> {
    println!("解压文件{}到{}", path, to);
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(to)?;
    Ok(())
}
