# Git Scanner


[![Docs](https://docs.rs/git-scanner/badge.svg)](https://docs.rs/git-scanner)
[![Crates.io](https://img.shields.io/crates/d/git-scanner.svg)](https://crates.io/crates/git-scanner)
[![Crates.io](https://img.shields.io/crates/v/git-scanner.svg)](https://crates.io/crates/git-scanner)

based on [Polyglot Code Scanner](https://github.com/kornysietsma/polyglot-code-scanner) with changes:

 - keep git logic only.
 - publish to crates.io

examples:

```
use git_scanner::flare::FlareTreeNode;
use git_scanner::git::GitCalculator;
use git_scanner::git_logger::GitLogConfig;
use git_scanner::{file_walker, IndicatorCalculator};
use std::path::PathBuf;

pub fn by_path(root: PathBuf) -> FlareTreeNode {
    let mut tics: Vec<Box<dyn IndicatorCalculator>> = vec![];
    let calculator = Box::new(GitCalculator::new(
        GitLogConfig::default().include_merges(true).since_years(3),
        true,
    ));

    tics.push(calculator);

    let mut tree = file_walker::walk_directory(&root, &mut tics).unwrap();

    for tic in tics {
        if let Some(metadata) = tic.metadata().unwrap() {
            tree.add_data(tic.name() + "_meta", metadata);
        }
    }

    return tree;
}
```


todo:

 - [ ] add git clone.
 - [ ] add history.
 - [ ] merge cmd from coco.



Copyright 2019 Kornelis Sietsma

Copyright © 2021 Inherd Group

Licensed under the Apache License, Version 2.0 - see LICENSE.txt for details

