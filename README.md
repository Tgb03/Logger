# Logger

This app aims to streamline using logs to get all the data from them automatically.

# Features

## AutoSplitter: 
- [x] Real time splitter
- [x] Automatically read and show end times of runs
- [x] Calculate total time of runs and theoretical best
- [ ] Remove broken splits or splits that are supposed to be ignored
- [x] Saving runs

## WardenMapper:
- [x] Automatically find keys.
- [x] Automatically find objective items.

## Full game/rundown speedruns:
- [x] Automatically give time.
- [ ] Automatically check if run is complete.
- [x] Calculate splits.
- [ ] Calculate theoretical best.

# What each setting does:

### Livesplitter Settings

- `Show Actual Splits` Shows the splits in each run. Simply turn it off if you don't want to see the splits.

- `LiveWindow Transparency` A value between 0 and 1 that shows how transparent the app is. 1 is opaque, 0 is see-through.

- `Show Game Splitter` Shows the full rundown/game splits. Use this if you are running a GTFO% or Rundown% run.

- `Show Run Counter` Shows a run counter in the livesplitter along with a seed counter. The run counter is how many times u dropped into a level (resets included) while the seed counter is how many unique seeds you got.

- `Path to logs folder` The path for where the game saves your logs. Modify this if you have some weird setup.

- `X position` The default X position for the livesplitter on your screen.

- `Y position` The default Y position for the livesplitter on your screen.

- `Compare to saved record` Whether or not the current run you are in gets compared to the saved record on the app.

- `Compare to best splits` Whether or not the current run you are in gets compared to the best splits saved on the app.

- `Splitter max length` how many splits are shown max in the livesplitter.
- `Game splitter max length` how many splits are shown max in the gamesplitter.

### Mapper Settings

- `Open LevelView folder` show the folder in which the levelview files are stored
- `Open examples for LevelView` open a link to a few examples for how these files look so you can make your own
- `Show Mapper in live splitter` show the mapper in the live splitter
- `Show objective items in live splitter` show objectives in the mapper
- `Show code guess` show code guess part of the livesplitter
- `Code guess number of lines` number of shown lines
- `Code guess number of words per line` number of words per line

### General

- `Automatic Loading of runs` automatically load file save data from PC.

# How to use:

Basic tutorial until I make a slightly better one:

https://youtu.be/rvlCpxyXw_k

Open the app and press the "Input Speedrun Logs..." button on the top left, now select all the logs you wish to input.
Now all the runs have been opened, you can play around and see what each button does, if you wish to restart simply press the button again and reinput the same logs.
Once you are done checking out the runs and selecting whether the run attempted secondary or overload, you can save each one individiually or all of them.
You can then press the check saved runs to see more data about each level and run such as best splits, total time spent running that level or other information.

# Mapper:

Simply turn on the Mapper feature and you should see all keys mapped. 
For GTFO here are all the key maps kept updated by d4rkeva currently made:
https://steamcommunity.com/sharedfiles/filedetails/?id=3166671266

If one map is missing please message me and I will see if I can obtain a map or poke others for it. I however cannot confirm anything.

Important note is the mapper loads data from https://github.com/Tgb03/Logger/blob/master/collectable_maps.ron so if you would like to add support for a level, simply make a pull request and I will merge it.

# Beware:

There are issues with the app which can cause the times to be innacurate, always double check with a VOD for runs you are submitting and take the Theoretical Best Time with a grain of salt. It may be innacurate.
