# Logu-Rs





## Input



	{
		"inputs": [
			"Switch",
			"Lichtschranke"
		],
		"outputs": [
			"LICHT"
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
