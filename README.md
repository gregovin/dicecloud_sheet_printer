# dicecloud_sheet_printer
A character sheet printer for Dice cloud v2. Designed for 5e characters, output may look slightly bootleg(pdf is hard ok)

An example sheet can be found at character_sheet.pdf

## Installing and using Dicecloud sheet printer
First, go to the releases tab and download the latest release for your platform. Then unpack the archive in a location of your choice. On windows you can just double click the dicecloud_sheet_printer.bat file and on linux you can run the .sh script in your terminal or just run `./targets/release/dicecloud_sheet_printer` from the main folder. Then it will display a series of propmts. First, it will ask for username and password. If the character sheet is publicly viewable, you can just press enter twice and then type Y when asked if you want to continue. Otherwise, type in your username and password. This should succeed if you did so correctly. If it fails, type n and retry. Currently you can't retry except by exiting the program and then restarting it. You will then be asked for the character id. To find this, look at your character URL. It should look like this `https://beta.dicecloud.com/character/[random seeming stuff]/name`. Coppy the random seeming stuff and paste. For example, the character id for https://beta.dicecloud.com/character/tARF8SRLPtQq9cjuw/jsonTest, the test character I have been using, would be `tARF8SRLPtQq9cjuw`. It will do it's thing for a bit, and then ask you what you want the output to be. It puts the printed output in the sheet_outputs folder. You just enter the name of the file you want. The program adds .pdf if needed(so if you didn't). It will then render a pdf at the specified location.

## Stuff that needs to be added before release
And spells(now in the latest dev version!), damage mults, and look at proficiency again(half proficiency is whack), magic items

## Homebrew Recognition
Hard Limits: Ability Scores and Saving throws are hardcoded to use the standard six
Soft Limits: Skills. While additional skills are supported, adding skills of type skill may cause printing errors
Everything else is in theory supported, with the following notes:
Custom Classes are supported so long as they use the class variable
Custom Races/Subraces are supported so long as they declare a constant with variable name "race" or "subRace"
Custom backgrounds are in theory supported, as long as the feature or slotfiller containing the description has the tag background as its first tag(the description is expected to be in the description field)
Custom features, actions, and attacks should work out of the box, however attacks may not find their damage in some cases
Custom items are supported, and custom magic items will be supported when magic items are supported
## How to compile it yourself
You will need the rust toolchain, which can be installed following these [instructions](https://doc.rust-lang.org/book/ch01-01-installation.html)
Clone the repo, then cd into it and run `cargo run`. If you get a weird error complaining about openssl on linux, install the openssl development package for your platform(libssl-dev for ubuntu) and try again. At this point it should work (tm)