# Logu-Rs

Generates a truth table based on the input and output conditions.

Had to do it once in school like 1 or 2 years ago and wanted to have a tool that would do the job for the rest of my life.

And now it even takes json as input.

## What is a Truth Table

	We have 2 buttons -> input.
	    B1 and B2. 
	We have 1 LED -> output.
	    L1

	If B1 and B2 are pressed the LED is lit.
	    B1 && B2 -> L1 -> 1
	    
	If B1 or B2 is pressed but not the other one, nothing happens.
	    L1 -> 0

	In a table this could look something like this:

	╔═════════╤═════════╤══════════╗
	║ Input 0 │ Input 1 │ Output 0 ║
	╟━━━━━━━━━┼━━━━━━━━━┼━━━━━━━━━━╢
	║ 0       │ 0       │ 0        ║
	║ 0       │ 1       │ 0        ║
	║ 1       │ 0       │ 0        ║
	║ 1       │ 1       │ 1        ║
	╚═════════╧═════════╧══════════╝
	Table generated with this tool


	How will this operation look in code ? 

	Input = {"0 = 1", "1 = 1"} Output = "0=1"
	And we are done.

## Input



	{
		"inputs": [
			"Switch",
			"Sensor"
		],
		"outputs": [
			"LED"
		],
		"expressions": [
			{
				"inputs": [
					"0=1",
					"1=1"
				],
				"outputs": [
					"0=1"
				]
			}
		]
	}


## Command

./logu run --input [json_file_name]

## Output

	+--------+---------------+-------+
	| Switch | Lichtschranke | LICHT |
	+--------+---------------+-------+
	| 0      | 0             | 0     |
	+--------+---------------+-------+
	| 0      | 1             | 0     |
	+--------+---------------+-------+
	| 1      | 0             | 0     |
	+--------+---------------+-------+
	| 1      | 1             | 1     |
	+--------+---------------+-------+



## Future


The current version of the tool [v0.2.0] is working as intended. But only given valid input.
Therefore the next steps include but are not limited to:

- [ ] Implement input validation on json input
- [ ] Optional highlighting of parts of the table {like all lines that include a 1 in the output part}
- [ ] Clean up the code
	- [ ] Extract methods 
	- [ ] Remove redundant code
	- [ ] Error-Messages

