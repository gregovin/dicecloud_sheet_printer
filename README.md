# dicecloud_sheet_printer
A character sheet printer for Dice cloud v2. Designed for 5e characters, output may look slightly bootleg(pdf is hard ok)

An example sheet can be found at character_sheet.pdf


## How to use it(extremely beta)
when run via the command line\(there is no release yet, so you'll have to compile yourself\) it will display a series of propmts. First, it will ask for username and password. If the character sheet is publicly viewable, you can just press enter twice and then type Y when asked if you want to continue. Otherwise, type in your username and password. This should succeed if you did so correctly. Currently you can't retry except by exiting the program and then restarting it. You will then be asked for the character id. To find this, look at your character URL. It should look like this `https://beta.dicecloud.com/character/[random seeming stuff]/name`. Coppy the random seeming stuff and paste. For example, the character id for https://beta.dicecloud.com/character/tARF8SRLPtQq9cjuw/jsonTest, the test character I have been using, would be `tARF8SRLPtQq9cjuw`. From this point the program should proceed automatically (unless your character has an absurd amount of attacks). It will then render a pdf with the name character_sheet.pdf in the top level folder