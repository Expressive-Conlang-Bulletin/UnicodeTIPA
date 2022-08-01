#!/usr/bin/env wolframscript
(* ::Package:: *)

SetDirectory@If[$Notebooks, NotebookDirectory[], DirectoryName@$InputFileName]


Get@"UnicodeTIPA.wl"


SetOptions[$Output, CharacterEncoding -> $SystemCharacterEncoding];


test = "do\:0303\:02d1\:02e9 \:0283ida\:02d1\:02e5\:02e9 | \:0272e\:02e7 t\:0361s\:02b0\:0289\:02e5 mu\:02d0\:0272i\:02d1\:02e5\:02e9 t\:0361s\:02b0i\:02d1\:02e5\:02e9 \:0294owvis\:02e7 t\:0361s\:02b0\:0289j\:02e7 mowr\:0289\:0303\:02d1\:02e9 le\:02d1\:02e9 dud\:0320\:0361\:0292i\:02d0\:02e7\:02e5 | \:0272i\:02e5 nu\:02d0\:0292e\:02d0\:02e7\:02e5 t\:02b0u\:0292e\:02d0\:02e7\:02e5 t\:0320\:0361\:0283\:02b0i\:02e7 | fi\:0283e\:02d0\:02e7\:02e7 ra\:02d0k\:025bj\:02e5 but\:0320\:0361\:0283i\:02e5 \:0294ub\:02e7 | fi\:0283e\:02d0\:02e7\:02e7 muws\:0289\:02d0\:02e7\:02e5 ra\:02d0k\:025bj\:02e5 \:0272e\:02e5 \:2016";


Print@Tipafy@test


If[$Notebooks, ResetDirectory[]]
