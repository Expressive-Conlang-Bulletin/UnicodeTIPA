#!/usr/bin/env wolframscript
(* ::Package:: *)

SetDirectory@If[$Notebooks, NotebookDirectory[], DirectoryName@$InputFileName]


Get@"tipafy.wl"


SetOptions[$Output, CharacterEncoding -> $SystemCharacterEncoding];


If[Length@$ScriptCommandLine > 1,
	Replace[s_String :> Quiet@Print@Tipafy@s]@Last@$ScriptCommandLine
]


If[$Notebooks, ResetDirectory[]]
