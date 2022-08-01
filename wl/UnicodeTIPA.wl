(* ::Package:: *)

BeginPackage["UnicodeTIPA`"]


If[$Notebooks,
	SetDirectory@NotebookDirectory[]
]

SetDirectory@FileNameJoin@{ParentDirectory[], "data"}


CatenateTokens
Tipafy


Begin["`Private`"]


importTable = Import[#, {"TSV", "RawData"}, CharacterEncoding -> "UTF8"] &;


selectCRAs[h_] := If[#4==="", If[#5==="", Nothing, ToExpression@# -> h@#5], ToExpression@# -> h@#4]&;


`$Letters = selectCRAs[Letter] @@@ Join @@ importTable /@ {"IPAExtension.tsv", "LatinExtended.tsv","Greek.tsv"}


`$Marks = selectCRAs[Mark] @@@ Join@@ importTable /@ {"SpacingModifier.tsv","Arrow.tsv","GeneralPunctuation.tsv"}


`$Groups = selectCRAs[Mark] @@@ importTable@"Groups.tsv"


`$Combinators = With[{h = head |-> Switch[head, "L", CombineLeft, "LR", CombineBoth, _, Throw@#7]},
	If[#4==="", If[#5==="", Nothing, FromDigits@# -> h[#7]@#5], FromDigits@# -> h[#7]@#4]
]& @@@ importTable@"CombiningDiacriticalMarks.tsv"


ResetDirectory[]
If[$Notebooks,
	ResetDirectory[]
]


ApplyMacro[cs_][content_] := cs<>"{"<>content<>"}"


StringMatchQ[LetterCharacter]@";"


(* not so reliable *)
`$CSNamePattern = LetterCharacter|"@";
`$CSNameDelimiterPattern = Alternatives@@Characters@"{ }";

`$CatenateMode = "Space";

CatenateTokens[list_] := Switch[$CatenateMode,
	"Bracket",
		"{"<>StringRiffle[First/@list, "}{"]<>"}",
	"Space",
		Fold[
			If[StringMatchQ[___~~"\\"~~Except[`$CSNameDelimiterPattern]...]@#1 && StringMatchQ[`$CSNamePattern~~___]@#2, #1<>" "<>#2, #1<>#2]&,
			First/@list
		],
	_,
		Throw["Not Implemented"]
]


Tipafy::unknownchar = "Unknown character ``";

HandleUnknown[c_] := (
	Message[Tipafy::unknownchar, c];
	Unknown@c
)


`$ToneLigature = "Ligature";
`$ToneLigatureRules := {
	"\\tone{"~~d:DigitCharacter~~d_~~"}" :> d,
	"\\tone{"~~ds:DigitCharacter..~~"}" :> ds
}[[
	Switch[`$ToneLigature, "Ligature", 1;;2, "Catenate", 2]
]]


Tipafy[str_String] := With[{codes = ToCharacterCode[str, "Unicode"]}, Module[{tokens, grouped, combined, toneLigatured},
	tokens = Replace[codes, Join[
		`$Letters,
		`$Marks,
		`$Combinators,
		{
			l_ :> Letter@FromCharacterCode@l /; 97<=l<=122,
			32 :> Mark@"\\,",
			10 :> Mark@"\\\\",
			9 :> Mark@"\\quad",
			c_ :> HandleUnknown@c
		}
	], 1];
	grouped = tokens //SequenceReplace[
		MapAt[Map[Unknown], {;;, 1}]@`$Groups
	];
	toneLigatured = grouped //If[`$ToneLigature === "Keep", Identity, SequenceReplace[
		{tones:Repeated[Mark@_?(StringMatchQ["\\tone{"~~DigitCharacter..~~"}"]), {2, Infinity}]} :>
			Mark[StringReplace[`$ToneLigatureRules]@*First/@{tones} //ApplyMacro["\\tone"]]
	]];
	combined = FixedPoint[
		SequenceReplace[{
			{(Letter|Combined)@l_, CombineLeft@cl_} :> Combined@ApplyMacro[cl][l],
			{l:(_Letter|_Combined), lm___Mark, CombineBoth@clr_, rm___Mark, r:(_Letter|_Combined)} :>
				Combined@ApplyMacro[clr]@CatenateTokens@{l, lm, rm, r}
		}]
	, toneLigatured];
	CatenateTokens@Replace[combined, Unknown@c_Integer :> Unknown@FromCharacterCode@c, 1]
] ]


End[]


EndPackage[]
