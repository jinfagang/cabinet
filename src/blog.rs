extern crate chrono;
extern crate colored;

use self::chrono::Local;
use colored::*;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn generate_blog_template(use_date: bool, title: &str) {
    let mut save_file_title = str::replace(title, " ", "_").to_string();

    let date = Local::now();
    let date_str = date.format("%Y_%m_%d_%H").to_string();
    let date_time_str = date.format("%Y-%m-%d %H:%M:%S").to_string();

    if use_date {
        println!("\n=> use date as title prefix {}", "ON".yellow().bold());
        save_file_title = format!("{}_{}.md", date_str, save_file_title);
    } else {
        println!("\n=> use date as title prefix {}", "OFF".yellow().bold());
        save_file_title = format!("{}.md", save_file_title);
    }

    println!("=> generate blog with name: {:?}", save_file_title);

    let content = format!("---
title: {}
date: {}
category: 默认分类
---
本文介绍 {}
<!-- more -->

# {}
> This article was original written by Jin Tian, you are welcomed to repost, first publish at http://blog.tsuai.cn. Please keep this copyright info, thanks, any question could be asked via wechat: `jintianiloveu` ",
                          title, date_time_str, title, title);

    let path = Path::new(&save_file_title);
    let display = path.display();
    let _file = match File::open(&path) {
        Err(_) => {
            let mut _file = match File::create(&path) {
                Err(why) => panic!("=> couldn't create {}: {}", display, why.description()),
                Ok(mut _file) => match _file.write_all(content.as_bytes()) {
                    Err(why) => panic!(
                        "=> couldn't write to {}: {}",
                        display,
                        why.description().red()
                    ),
                    Ok(_) => println!(
                        "=> successfully wrote to {}",
                        display.to_string().yellow().bold()
                    ),
                },
            };
        }
        Ok(_file) => println!("=> file already exist!"),
    };
}
