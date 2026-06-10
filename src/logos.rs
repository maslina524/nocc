// 35 x 19
pub const LINES: usize = 23;

pub const WIN11: [&'static str; 19] = [
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW"
];

pub const WIN10_8: [&'static str; 19] = [
    "\x1b[1;34m                                 .,",
    "\x1b[1;34m                      ..,,:;+wwWWWW",
    "\x1b[1;34m        ....,+:;   wwWWWWWWWWWWWWWW",
    "\x1b[1;34m...,wwwWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34mWWWWWWWWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34m```*wwwWWWWWWWWW   WWWWWWWWWWWWWWWW",
    "\x1b[1;34m        ``'\\\\*::   wWWWWWWWWWWWWWWW",
    "\x1b[1;34m                      ````''**::wWW",
    "\x1b[1;34m                                 ``"
];

pub const WIN7: [&'static str; 19] = [
    "",
    "\x1b[1;31m        wWWWWWWWWWw                ",
    "\x1b[1;31m       wWWWWWWWWWWWW               ",
    "\x1b[1;31m      wWWWWWWWWWWWWw  \x1b[1;32mw--      --/w",
    "\x1b[1;31m      WWWWWWWWWWWWw  \x1b[1;32mwWWWWWWWWWWWWW",
    "\x1b[1;31m     wWWWWWWWWWWWW  \x1b[1;32mwWWWWWWWWWWWWW'",
    "\x1b[1;31m    'WWWWWWWWWWWWw  \x1b[1;32mWWWWWWWWWWWWW' ",
    "\x1b[1;31m    W/--      --w  \x1b[1;32mwWWWWWWWWWWWWW  ",
    "\x1b[1;34m                   \x1b[1;32m'*WWWWWWWWWW*'  ",
    "\x1b[1;34m    wWWWWWWWWWw      \x1b[1;32m'*WWWWWWW*'   ",
    "\x1b[1;34m   wWWWWWWWWWWWW                   ",
    "\x1b[1;34m  wWWWWWWWWWWWWw   \x1b[1;33mw--      --/w   ",
    "\x1b[1;34m  WWWWWWWWWWWWw   \x1b[1;33mwWWWWWWWWWWWWW   ",
    "\x1b[1;34m wWWWWWWWWWWWW   \x1b[1;33mwWWWWWWWWWWWWW'   ",
    "\x1b[1;34m'WWWWWWWWWWWWw   \x1b[1;33mWWWWWWWWWWWWW'    ",
    "\x1b[1;34mW/--      --w   \x1b[1;33mwWWWWWWWWWWWWW     ",
    "\x1b[1;34m                \x1b[1;33m'*WWWWWWWWWW*'     ",
    "\x1b[1;34m                  \x1b[1;33m'*WWWWWWW*'      ",
    ""
];