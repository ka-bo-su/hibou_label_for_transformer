/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limit
 */


use std::path::Path;

use clap::ArgMatches;


use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::htf::interface::parse_htf_file;
use crate::trace_manip::mutate::remove_actions::generate_remove_actions_mutant;


pub fn cli_mutate_remove_actions(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {
            let htf_file_path = matches.value_of("htf").unwrap();
            match parse_htf_file(&gen_ctx, htf_file_path) {
                Err(e) => {
                    return (vec![e.to_string()],1);
                },
                Ok( (co_localizations,multi_trace) ) => {
                    // ***
                    let parent_folder : Option<&str>;
                    match matches.value_of("parent_folder") {
                        None => {
                            parent_folder = None;
                        },
                        Some( folder_name ) => {
                            parent_folder = Some( folder_name );
                        }
                    }
                    // ***
                    let max_num_removes : u32 = if matches.is_present("max_num_removes") {
                        let extracted = matches.value_of("max_num_removes").unwrap();
                        let content_str : String = extracted.chars().filter(|c| !c.is_whitespace()).collect();
                        content_str.parse::<u32>().unwrap()
                    } else {
                        1
                    };
                    // ***
                    let mutant_name : String = if matches.is_present("name") {
                        let extracted = matches.value_of("name").unwrap();
                        extracted.chars().filter(|c| !c.is_whitespace()).collect()
                    } else {
                        let mu_name : &str = Path::new(htf_file_path).file_stem().unwrap().to_str().unwrap();
                        format!("{}_remove_actions_mutant",mu_name)
                    };
                    // ***
                    let mutant_file_path = generate_remove_actions_mutant(&gen_ctx,
                                                                          &co_localizations,
                                                                          &multi_trace,
                                                                          parent_folder,
                                                                          &mutant_name,
                                                                          max_num_removes);
                    // ***
                    let mut ret_print = vec![];
                    ret_print.push( "GENERATED REMOVE ACTIONS MUTANT".to_string());
                    ret_print.push( "FOR MULTITRACE".to_string());
                    ret_print.push( format!("from file '{}'",htf_file_path) );
                    ret_print.push( format!("into file '{}'",mutant_file_path) );
                    ret_print.push( "".to_string());
                    return (ret_print,0);
                }
            }
        }
    }
}


