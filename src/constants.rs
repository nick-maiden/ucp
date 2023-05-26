pub const REMOTE: &str = "z5256318@login.cse.unsw.edu.au";
pub const GUESS_FLAG: &str = "-g";
pub const HELP_FLAG: &str = "-h";
pub const NUM_ARGS: usize = 2;

pub const INVALID_ARGS_MSG: &str = 
"Invalid number of arguments provided. Run with -h flag for help.";

pub const HELP_MSG: &str = 
"
DESCRIPTION:
    ucp - copy current working directory to cse account at unsw

USAGE:
    ucp <target/directory/in/remote>

FLAGS:
    -g
        usage: ucp -g
        guess remote directory based on local working directory
        assumes local working directory is within uni/ directory

    -h
        help flag, output this help message

";