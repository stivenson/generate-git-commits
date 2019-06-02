  

#  Generate-Git-Commits

[![Build Status](https://travis-ci.com/stivenson/generate-git-commits.svg?branch=master)](https://travis-ci.com/stivenson/generate-git-commits)

App to generate commit history of any folder with code or others files; The commits' date will be to according to the last update of each found file.

  
  

*this app was created with [Rust-Lang](https://www.rust-lang.org/)*

  

  

##  Getting Started

  

To run this code locally, is necessary that you install Rust v1.35+

  

  

###  Prerequisites

  

- Use Mac _(Of moment this project only have support to Mac OS systems, but I promise to upload support for other systems soon)_.

  
  

- Install [git](https://git-scm.com/).

  
  

- Install Rust following the [official site's instructions](https://www.rust-lang.org/learn/get-started).

  

###  Installing

After cloned the project, run this two commands:

  1. ``` bash cd <path to project>/generate-git-commits ```
  2.  ``` bash cargo build ```
  
  ----------------------------
|:heavy_exclamation_mark: Caution; The next command delete the .git folder, if there is found in the specified directory. check first if this is what you want. |  |
|-------------------------------------------------------------------------------------------------|--|
|                                                                                                 |  |

  3.  ```./target/debug/generate-git-commits  /<here replace by absolute path to your project>/```

  

_Here is necessary replace by absolute path of project to which you want to generate the commits_

  

  

And ready, you just have to wait for the process to finish. :tada::+1:

  

  

##  Running the tests

> The tests are in development.

  

  

##  Contributing


Any contribution is welcome. I am currently learning rust and this app is my first real project with this language.

  

  

##  Authors



*  **Stivenson Rincon** - *Full Stack developer* - [Stivenson](https://github.com/Stivenson)

 
See also the list of [contributors](contributing.md) who participated in this project.

## Release History

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2019-05-31  | v0.1.0  | Initial release. |


 
##  License

 

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE) file for details.

  

##  Acknowledgments


* To [Cuemby](http://cuemby.com/), for continuing to motivate me to learn new things
