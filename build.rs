// MIT License
//
// Copyright (c) 2019 Daniele Esposti
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate lalrpop;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const GRAMMAR_FILES: [&str; 1] = ["query"];
const GRAMMAR_DIR: &str = "src/grammar";
const OUTPUT_DIR: &str = "src";
const COMMON_GRAMMAR: &str = "common.lalrpop.template";

fn main() {
    let common_content;
    {
        let mut common_path = PathBuf::from(GRAMMAR_DIR);
        common_path.push(COMMON_GRAMMAR);

        common_content = fs::read(&common_path).unwrap_or_else(|_| panic!("Cannot read {:?}", common_path));
    }

    for grammar_file in GRAMMAR_FILES.iter() {
        let mut src_path = PathBuf::from(GRAMMAR_DIR);
        src_path.push(format!("{}.lalrpop.template", grammar_file));

        let mut dst_path = PathBuf::from(OUTPUT_DIR);
        dst_path.push(format!("{}.lalrpop", grammar_file));

        let grammar_content = fs::read(&src_path).unwrap_or_else(|_| panic!("Cannot read {:?}", src_path));

        let mut file =
            File::create(&dst_path).unwrap_or_else(|_| panic!("Cannot create file {:?}", dst_path));
        file.write_all(&common_content)
            .unwrap_or_else(|_| panic!("Failed writing into {:?}", dst_path));
        file.write_all(&grammar_content)
            .unwrap_or_else(|_| panic!("Failed writing into {:?}", dst_path));
    }

    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process()
        .unwrap()
}
