extern crate clap;
extern crate colored;


use clap::{Arg, App, SubCommand, ArgMatches};
use colored::*;
use std::{process};

mod blog;

fn parse_args() -> ArgMatches <'static> {
    let matches = App::new("Cabinet")
        .version("1.0.1")
        .about("ultimate tool box, standby at work.")
        .author("Lucas Jin")
        .subcommand(SubCommand::with_name("git")
            .about("Git simplify command, quick push to remote.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("comment")
                .help("comment msg of git changes.")
                .index(1)))

        .subcommand(SubCommand::with_name("blog")
            .about("Blog template generate, for my quick writing.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("title")
                .help("title of blog")
                .index(1))
            .arg(Arg::with_name("date")
                .short("d")
                .help("using date as prefix or not.")))

        .subcommand(SubCommand::with_name("code")
            .about("Generate codes project quickly.")
            .version("0.1")
            .author("Lucas Jin")
            .arg(Arg::with_name("name")
                .help("name of project.")
                .index(1)))
        .get_matches();
    return matches;
}


fn main() {
    println!("{} - {}", "Cabinet".yellow().bold(), "The Ultimate Tool Box".yellow().bold());
    println!("written by {} with {}.", "Lucas Jin".green().bold(), "Rust".red().bold());
    println!("{} \nwelcome folk and star!", "https://github.com/jinfagang/cabinet");

    let matches = parse_args();

    if matches.is_present("git") {
        println!("using {} module", "git".yellow().bold());

        println!("we are not support git anymore, removed from cabinet.");
//
//        let mut comment = "[cabinet] just add some files";
//        if let Some(git_matches) = matches.subcommand_matches("git") {
//            if git_matches.is_present("comment") {
//                comment = git_matches.value_of("comment").unwrap();
//                println!("\nyour comment msg: [{}]", comment.yellow().bold());
//            } else {
//                println!("\nno comment, using default: {}", comment.yellow().bold());
//            }
//        }
//
//        let path_buf = env::current_dir().unwrap();
//        let _repo = match Repository::open(&path_buf) {
//            Ok(repo) => {
//
//                println!("{}:", "last commit message");
//                let commit = git::find_last_commit(&repo).expect("Couldn't find last commit");
//                git::display_commit(&commit);
//
//                let _add_r = match git::add_and_commit(&repo, &path_buf, &comment) {
//                    Ok(_add_r) => {
//                        println!("new files added.");
//                    },
//                    Err(e) => {
//                        println!("error occurred when adding files: {}", e.to_string().red());
//                    }
//                };
//
//                let _remote = match repo.find_remote("origin") {
//                    Ok(r) => {
//                        let url = r.url().unwrap();
//                        println!("pushing to remote: {}", url.yellow().bold());
//                        let _r = match git::push(&repo, &url) {
//                            Ok(()) => {
//                                println!("Done! pushing succeed!")
//                            },
//                            Err(e) => {
//                                println!("Some error while pushing codes. {}", e.to_string());
//                            }
//                        };
//
//                    },
//                    Err(e) => {
//                        println!("error: {}", e.to_string());
//                        println!("maybe remote not add yet, try {}", "git remote add url".yellow().bold());
//                    }
//                };
//            },
//            Err(e) => {
//                panic!("failed to open: {}", e)
//            },
//        };


    } else if matches.is_present("blog") {
        println!("using {} module", "blog".yellow().bold());

        let mut _title = "no_title";
        let mut is_date = false;
        if let Some(blog_matches) = matches.subcommand_matches("blog") {
            if blog_matches.is_present("date") {
                is_date = true;
            }

            if blog_matches.is_present("title") {
                _title = blog_matches.value_of("title").unwrap();
            } else {
                println!("no title provided.");
                process::exit(1);
            }

            blog::generate_blog_template(is_date, _title);
        }


    } else if matches.is_present("code") {
        println!("using {} module", "code".yellow().bold());



    }
}
