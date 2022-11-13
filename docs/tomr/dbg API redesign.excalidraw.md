---

excalidraw-plugin: parsed
tags: [excalidraw]

---
==⚠  Switch to EXCALIDRAW VIEW in the MORE OPTIONS menu of this document. ⚠==


# Text Elements
pid.cont()
dbgid.cont() ^Lk6nZiSl

Pid and Dbgid should both have cont() method ^wr1Ngb2R

Debugee has cont() method,
Dbgid and Pid have TryInto<Debugee> ^f65GQx2b

Need to replace pid.try_into::<Debugee>
 ^HzG3hIkd

Pid and Dbgid have DebugContext trait,
which mimics methods of Debugee and calls them fallibly
after a try_into::<Debugee>() ^tXDTb3Yx

Use macro to generate code for DebugeContext impl,
as every fn impl is the same:
- Call try_into::<Debugee>() and return if error
- Call Debugee fn with the new owned Debugee ^X39YwGpD

Checking that Debugee is still valid
(is still in DEBUGEES)
must be done a second time - 
- Once for the TryInto
- Once for for Debugee's method itself.
This is not nice. ^33znno2U

Wanted Behaviour (one of 'em) ^CUgadFoa

cont(pid)
cont(dbgid) ^PdtlbRR2

API debugging fns are Generic,
and receive only types which TryInto<Dbgid>.
TryInto<Dbgid> reads DEBUGEES. ^KSh0olVf

If another detail about the debugged process is needed
other than the given data and the Dbgid,
an additional search in DEBUGEES is needed.
Therefore DEBUGEES should be a HashMap with Dbgid as key -
so that additional searches after the initial TryInto<Dbgid> will be fast. ^9uC7Ics8

- Convert DebugeeList inner to a HashMap<Dbgid, Debugee>
- Make dbg API calls generic for every TryInto<Dbgid> ^GrMJZFKt

%%
# Drawing
```json
{
	"type": "excalidraw",
	"version": 2,
	"source": "https://excalidraw.com",
	"elements": [
		{
			"type": "text",
			"version": 136,
			"versionNonce": 1769708502,
			"isDeleted": false,
			"id": "Lk6nZiSl",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 114.13235294117658,
			"y": -280.1107536764706,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 112,
			"height": 50,
			"seed": 1896128586,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "_lgrD_FAIhtwkN8lMvcTR",
					"type": "arrow"
				},
				{
					"id": "eScC0aHzIqB2F2SSgoPcT",
					"type": "arrow"
				}
			],
			"updated": 1668377571822,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "pid.cont()\ndbgid.cont()",
			"rawText": "pid.cont()\ndbgid.cont()",
			"baseline": 43,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "pid.cont()\ndbgid.cont()"
		},
		{
			"type": "arrow",
			"version": 965,
			"versionNonce": 646295830,
			"isDeleted": false,
			"id": "_lgrD_FAIhtwkN8lMvcTR",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 167.78745038199946,
			"y": -216.58134191176472,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 19.978147216904006,
			"height": 113,
			"seed": 223827082,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668377571823,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "Lk6nZiSl",
				"focus": 0.14968566614690956,
				"gap": 13.529411764705884
			},
			"endBinding": {
				"elementId": "wr1Ngb2R",
				"focus": -0.05193227153872985,
				"gap": 12.823529411764724
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					19.978147216904006,
					113
				]
			]
		},
		{
			"type": "text",
			"version": 164,
			"versionNonce": 194758986,
			"isDeleted": false,
			"id": "wr1Ngb2R",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -24.75,
			"y": -90.7578125,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 458,
			"height": 25,
			"seed": 599950998,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "_lgrD_FAIhtwkN8lMvcTR",
					"type": "arrow"
				},
				{
					"id": "7thGaaMZpHMZ2-TgYPNMQ",
					"type": "arrow"
				}
			],
			"updated": 1668376749735,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Pid and Dbgid should both have cont() method",
			"rawText": "Pid and Dbgid should both have cont() method",
			"baseline": 18,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Pid and Dbgid should both have cont() method"
		},
		{
			"type": "text",
			"version": 122,
			"versionNonce": 1871079178,
			"isDeleted": false,
			"id": "f65GQx2b",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 280.25,
			"y": 36.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 380,
			"height": 50,
			"seed": 1852468246,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "7thGaaMZpHMZ2-TgYPNMQ",
					"type": "arrow"
				},
				{
					"id": "-Y50ZTpUd5bJrMdzimA-O",
					"type": "arrow"
				},
				{
					"id": "zG4s4Bpxh3QOJrJYiAhNY",
					"type": "arrow"
				}
			],
			"updated": 1668377238381,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Debugee has cont() method,\nDbgid and Pid have TryInto<Debugee>",
			"rawText": "Debugee has cont() method,\nDbgid and Pid have TryInto<Debugee>",
			"baseline": 43,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Debugee has cont() method,\nDbgid and Pid have TryInto<Debugee>"
		},
		{
			"type": "arrow",
			"version": 42,
			"versionNonce": 9274826,
			"isDeleted": false,
			"id": "7thGaaMZpHMZ2-TgYPNMQ",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 304.25,
			"y": -54.7578125,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 96,
			"height": 78,
			"seed": 913743882,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668376329276,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "wr1Ngb2R",
				"focus": -0.29084041548630785,
				"gap": 11
			},
			"endBinding": {
				"elementId": "f65GQx2b",
				"focus": -0.10522648083623695,
				"gap": 13
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					96,
					78
				]
			]
		},
		{
			"type": "text",
			"version": 64,
			"versionNonce": 1445745290,
			"isDeleted": false,
			"id": "HzG3hIkd",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 220.25,
			"y": 197.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 393,
			"height": 50,
			"seed": 355351690,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "-Y50ZTpUd5bJrMdzimA-O",
					"type": "arrow"
				},
				{
					"id": "LhDWtefAIcBiIY077q3lm",
					"type": "arrow"
				}
			],
			"updated": 1668376742357,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Need to replace pid.try_into::<Debugee>\n",
			"rawText": "Need to replace pid.try_into::<Debugee>\n",
			"baseline": 43,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Need to replace pid.try_into::<Debugee>\n"
		},
		{
			"type": "text",
			"version": 161,
			"versionNonce": 1409320906,
			"isDeleted": false,
			"id": "tXDTb3Yx",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 200.25,
			"y": 296.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 548,
			"height": 75,
			"seed": 1252086038,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "LhDWtefAIcBiIY077q3lm",
					"type": "arrow"
				},
				{
					"id": "67eOW21LaQjf4v93e3Zge",
					"type": "arrow"
				}
			],
			"updated": 1668376744421,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Pid and Dbgid have DebugContext trait,\nwhich mimics methods of Debugee and calls them fallibly\nafter a try_into::<Debugee>()",
			"rawText": "Pid and Dbgid have DebugContext trait,\nwhich mimics methods of Debugee and calls them fallibly\nafter a try_into::<Debugee>()",
			"baseline": 68,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Pid and Dbgid have DebugContext trait,\nwhich mimics methods of Debugee and calls them fallibly\nafter a try_into::<Debugee>()"
		},
		{
			"type": "text",
			"version": 244,
			"versionNonce": 1190198922,
			"isDeleted": false,
			"id": "X39YwGpD",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 123.25,
			"y": 435.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 528,
			"height": 100,
			"seed": 536257110,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "67eOW21LaQjf4v93e3Zge",
					"type": "arrow"
				}
			],
			"updated": 1668376744421,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Use macro to generate code for DebugeContext impl,\nas every fn impl is the same:\n- Call try_into::<Debugee>() and return if error\n- Call Debugee fn with the new owned Debugee",
			"rawText": "Use macro to generate code for DebugeContext impl,\nas every fn impl is the same:\n- Call try_into::<Debugee>() and return if error\n- Call Debugee fn with the new owned Debugee",
			"baseline": 93,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Use macro to generate code for DebugeContext impl,\nas every fn impl is the same:\n- Call try_into::<Debugee>() and return if error\n- Call Debugee fn with the new owned Debugee"
		},
		{
			"type": "arrow",
			"version": 31,
			"versionNonce": 402854294,
			"isDeleted": false,
			"id": "-Y50ZTpUd5bJrMdzimA-O",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 420.25,
			"y": 94.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 89,
			"seed": 142397130,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668376740519,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "f65GQx2b",
				"focus": 0.2288111078970205,
				"gap": 8
			},
			"endBinding": {
				"elementId": "HzG3hIkd",
				"focus": -0.09004394435580934,
				"gap": 14
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-15,
					89
				]
			]
		},
		{
			"type": "arrow",
			"version": 26,
			"versionNonce": 794784534,
			"isDeleted": false,
			"id": "LhDWtefAIcBiIY077q3lm",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 382.25,
			"y": 253.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 8,
			"height": 33,
			"seed": 135818966,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668376742357,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "HzG3hIkd",
				"focus": 0.13321864013763182,
				"gap": 6
			},
			"endBinding": {
				"elementId": "tXDTb3Yx",
				"focus": -0.3939199314921858,
				"gap": 10
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-8,
					33
				]
			]
		},
		{
			"type": "arrow",
			"version": 28,
			"versionNonce": 1342452182,
			"isDeleted": false,
			"id": "67eOW21LaQjf4v93e3Zge",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 354.25,
			"y": 381.2421875,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 23,
			"height": 45,
			"seed": 141336662,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668376744421,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "tXDTb3Yx",
				"focus": 0.3265112753458404,
				"gap": 10
			},
			"endBinding": {
				"elementId": "X39YwGpD",
				"focus": -0.29754412893323096,
				"gap": 9
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-23,
					45
				]
			]
		},
		{
			"type": "arrow",
			"version": 299,
			"versionNonce": 1677756630,
			"isDeleted": false,
			"id": "zG4s4Bpxh3QOJrJYiAhNY",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 274.8200775419119,
			"y": 95.41335583543014,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 251.75604086733227,
			"height": 103.05380586477622,
			"seed": 2046656202,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668377462961,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "f65GQx2b",
				"focus": 0.4458894599325635,
				"gap": 9.171168335430139
			},
			"endBinding": {
				"elementId": "33znno2U",
				"focus": -0.34829671528049966,
				"gap": 21.588235294117737
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-251.75604086733227,
					103.05380586477622
				]
			]
		},
		{
			"type": "text",
			"version": 443,
			"versionNonce": 2031229718,
			"isDeleted": false,
			"id": "33znno2U",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -277.4668902648778,
			"y": 220.0553969943241,
			"strokeColor": "#c92a2a",
			"backgroundColor": "transparent",
			"width": 394,
			"height": 150,
			"seed": 2143217302,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "zG4s4Bpxh3QOJrJYiAhNY",
					"type": "arrow"
				}
			],
			"updated": 1668377458500,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Checking that Debugee is still valid\n(is still in DEBUGEES)\nmust be done a second time - \n- Once for the TryInto\n- Once for for Debugee's method itself.\nThis is not nice.",
			"rawText": "Checking that Debugee is still valid\n(is still in DEBUGEES)\nmust be done a second time - \n- Once for the TryInto\n- Once for for Debugee's method itself.\nThis is not nice.",
			"baseline": 143,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Checking that Debugee is still valid\n(is still in DEBUGEES)\nmust be done a second time - \n- Once for the TryInto\n- Once for for Debugee's method itself.\nThis is not nice."
		},
		{
			"type": "text",
			"version": 139,
			"versionNonce": 1658983818,
			"isDeleted": false,
			"id": "CUgadFoa",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -226.8786549707603,
			"y": -431.70930888802866,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 304,
			"height": 25,
			"seed": 758857482,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "eScC0aHzIqB2F2SSgoPcT",
					"type": "arrow"
				},
				{
					"id": "KdpFHZBFxll4AcKiuhN7H",
					"type": "arrow"
				}
			],
			"updated": 1668377619705,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Wanted Behaviour (one of 'em)",
			"rawText": "Wanted Behaviour (one of 'em)",
			"baseline": 18,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Wanted Behaviour (one of 'em)"
		},
		{
			"type": "arrow",
			"version": 383,
			"versionNonce": 635098186,
			"isDeleted": false,
			"id": "eScC0aHzIqB2F2SSgoPcT",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -5.026034096383455,
			"y": -394.4740147703817,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 111.94458000438856,
			"height": 102.420713747045,
			"seed": 1868647382,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668377619705,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "CUgadFoa",
				"focus": -0.2569838054924318,
				"gap": 12.235294117646959
			},
			"endBinding": {
				"elementId": "Lk6nZiSl",
				"focus": -0.27406110528972044,
				"gap": 13.952184382524933
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					111.94458000438856,
					102.420713747045
				]
			]
		},
		{
			"type": "text",
			"version": 41,
			"versionNonce": 452361110,
			"isDeleted": false,
			"id": "PdtlbRR2",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -323.3492432060543,
			"y": -276.4151912409698,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 106,
			"height": 50,
			"seed": 926015190,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "KdpFHZBFxll4AcKiuhN7H",
					"type": "arrow"
				},
				{
					"id": "F-_d7Me9MhAqOkx3CKfsc",
					"type": "arrow"
				}
			],
			"updated": 1668378000868,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "cont(pid)\ncont(dbgid)",
			"rawText": "cont(pid)\ncont(dbgid)",
			"baseline": 43,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "cont(pid)\ncont(dbgid)"
		},
		{
			"type": "arrow",
			"version": 288,
			"versionNonce": 1892530954,
			"isDeleted": false,
			"id": "KdpFHZBFxll4AcKiuhN7H",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -180.943107705079,
			"y": -400.35636771155805,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 92.97570773984796,
			"height": 109.41176470588232,
			"seed": 2030436810,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668377619705,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "CUgadFoa",
				"focus": 0.5535196424587866,
				"gap": 6.352941176470608
			},
			"endBinding": {
				"elementId": "PdtlbRR2",
				"focus": -0.500519513662299,
				"gap": 14.529411764705912
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-92.97570773984796,
					109.41176470588232
				]
			]
		},
		{
			"type": "text",
			"version": 241,
			"versionNonce": 869256598,
			"isDeleted": false,
			"id": "KSh0olVf",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -673.3949948400414,
			"y": -179.18643307103525,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 441,
			"height": 75,
			"seed": 1028189078,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "F-_d7Me9MhAqOkx3CKfsc",
					"type": "arrow"
				},
				{
					"id": "xJTt-YWUzAs39_6-BlOkU",
					"type": "arrow"
				}
			],
			"updated": 1668378008559,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "API debugging fns are Generic,\nand receive only types which TryInto<Dbgid>.\nTryInto<Dbgid> reads DEBUGEES.",
			"rawText": "API debugging fns are Generic,\nand receive only types which TryInto<Dbgid>.\nTryInto<Dbgid> reads DEBUGEES.",
			"baseline": 68,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "API debugging fns are Generic,\nand receive only types which TryInto<Dbgid>.\nTryInto<Dbgid> reads DEBUGEES."
		},
		{
			"type": "text",
			"version": 455,
			"versionNonce": 261395402,
			"isDeleted": false,
			"id": "9uC7Ics8",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -940.3949948400414,
			"y": 27.813566928964747,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 738,
			"height": 125,
			"seed": 356544522,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "xJTt-YWUzAs39_6-BlOkU",
					"type": "arrow"
				},
				{
					"id": "AlLfaYkMV95IgLYMfMoaG",
					"type": "arrow"
				}
			],
			"updated": 1668378238207,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "If another detail about the debugged process is needed\nother than the given data and the Dbgid,\nan additional search in DEBUGEES is needed.\nTherefore DEBUGEES should be a HashMap with Dbgid as key -\nso that additional searches after the initial TryInto<Dbgid> will be fast.",
			"rawText": "If another detail about the debugged process is needed\nother than the given data and the Dbgid,\nan additional search in DEBUGEES is needed.\nTherefore DEBUGEES should be a HashMap with Dbgid as key -\nso that additional searches after the initial TryInto<Dbgid> will be fast.",
			"baseline": 118,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "If another detail about the debugged process is needed\nother than the given data and the Dbgid,\nan additional search in DEBUGEES is needed.\nTherefore DEBUGEES should be a HashMap with Dbgid as key -\nso that additional searches after the initial TryInto<Dbgid> will be fast."
		},
		{
			"type": "arrow",
			"version": 94,
			"versionNonce": 49130198,
			"isDeleted": false,
			"id": "F-_d7Me9MhAqOkx3CKfsc",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -336.4106044228448,
			"y": -224.31736239115094,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 83.75970222867227,
			"height": 34.13092932011568,
			"seed": 277086474,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668378008559,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "PdtlbRR2",
				"focus": 0.008449253692564749,
				"gap": 13.228758169934565
			},
			"endBinding": {
				"elementId": "KSh0olVf",
				"focus": -0.27612829680294315,
				"gap": 11
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-83.75970222867227,
					34.13092932011568
				]
			]
		},
		{
			"type": "arrow",
			"version": 189,
			"versionNonce": 81649686,
			"isDeleted": false,
			"id": "xJTt-YWUzAs39_6-BlOkU",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -669.8249141308578,
			"y": -94.18643307103524,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 49.33492525392637,
			"height": 100.99999999999999,
			"seed": 593790858,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668378008559,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "KSh0olVf",
				"focus": 0.8121672610740682,
				"gap": 10
			},
			"endBinding": {
				"elementId": "9uC7Ics8",
				"focus": -0.4719347564177813,
				"gap": 21
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-49.33492525392637,
					100.99999999999999
				]
			]
		},
		{
			"type": "text",
			"version": 198,
			"versionNonce": 203655818,
			"isDeleted": false,
			"id": "GrMJZFKt",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -1068.2325298540463,
			"y": 319.94177205717017,
			"strokeColor": "#1864ab",
			"backgroundColor": "transparent",
			"width": 593,
			"height": 50,
			"seed": 1785287318,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "AlLfaYkMV95IgLYMfMoaG",
					"type": "arrow"
				}
			],
			"updated": 1668378238207,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "- Convert DebugeeList inner to a HashMap<Dbgid, Debugee>\n- Make dbg API calls generic for every TryInto<Dbgid>",
			"rawText": "- Convert DebugeeList inner to a HashMap<Dbgid, Debugee>\n- Make dbg API calls generic for every TryInto<Dbgid>",
			"baseline": 43,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "- Convert DebugeeList inner to a HashMap<Dbgid, Debugee>\n- Make dbg API calls generic for every TryInto<Dbgid>"
		},
		{
			"type": "arrow",
			"version": 46,
			"versionNonce": 1898293578,
			"isDeleted": false,
			"id": "AlLfaYkMV95IgLYMfMoaG",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -730.5854710305169,
			"y": 176.00059558658188,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 83.52941176470597,
			"height": 134.11764705882354,
			"seed": 2011293514,
			"groupIds": [],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1668378242140,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "9uC7Ics8",
				"focus": 0.25941994153178566,
				"gap": 23.187028657617134
			},
			"endBinding": {
				"elementId": "GrMJZFKt",
				"focus": -0.20530860196563494,
				"gap": 9.823529411764753
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-83.52941176470597,
					134.11764705882354
				]
			]
		},
		{
			"id": "lBuQq00C",
			"type": "text",
			"x": -57.49126492702317,
			"y": -37.930826319014386,
			"width": 6,
			"height": 25,
			"angle": 0,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"fillStyle": "hachure",
			"strokeWidth": 4,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"seed": 787153389,
			"version": 2,
			"versionNonce": 1146825389,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1668378920051,
			"link": null,
			"locked": false,
			"text": "i",
			"rawText": "i",
			"fontSize": 20,
			"fontFamily": 1,
			"textAlign": "left",
			"verticalAlign": "top",
			"baseline": 18,
			"containerId": null,
			"originalText": "i"
		}
	],
	"appState": {
		"theme": "dark",
		"viewBackgroundColor": "#ffffff",
		"currentItemStrokeColor": "#000000",
		"currentItemBackgroundColor": "transparent",
		"currentItemFillStyle": "hachure",
		"currentItemStrokeWidth": 4,
		"currentItemStrokeStyle": "solid",
		"currentItemRoughness": 0,
		"currentItemOpacity": 100,
		"currentItemFontFamily": 1,
		"currentItemFontSize": 20,
		"currentItemTextAlign": "left",
		"currentItemStrokeSharpness": "sharp",
		"currentItemStartArrowhead": null,
		"currentItemEndArrowhead": "arrow",
		"currentItemLinearStrokeSharpness": "round",
		"gridSize": null,
		"colorPalette": {}
	},
	"files": {}
}
```
%%