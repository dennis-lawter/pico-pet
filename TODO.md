# TODO

This list is not final. It will grow and evolve.

Done items should be removed from the list.

- Misc
  - Vibe setting
  - BUG: Sometimes the reboot sequence stalls
- Top bar
  - Tomato & Raspberry count
  - Displays health meter?
  - Displays time to next feeding?
    - probably 30h since the last feeding
  - A battery meter would be nice, but difficult.
- Idle Scene
  - Entered from the main scene
  - Setting to control time before idle
  - Attempt to insert a deep sleep cycle for 5 seconds if possible
  - Displays time, with backlight fully off
- Intro Scene
  - Only loaded if NVM is not initialized properly and must be reset
  - Remove the current code which checks for invalid NVM then autowrites default
  - Introduce pet
  - Explain the pomodoro method
  - Explain how feeding works
  - Force the player to save each of the setting components
    - Possibly describe how they each work (why must we have a clock?)
  - Finalize by saving to NVM and pet greeting scene
  - Remove the current code which checks for invalid NVM then autowrites
- Pomodoro Scene
  - Reward player with tomato
  - Reward player with tomato juice for pomo chains
  - Reward player with raspberry for cycle completion
- Eating Scene
  - Opening the scene after eating today says the pet cannot eat again.
    > I need to confirm this is how I want to do this.
    > I do want to minimize NVM writes,
    > but I don't want to punish a late night extra burst of energy.
    > Additionally, what happens to tomatoes gained at, say, 11PM?
    > Do they expire?
    > Do they become juice?
    > Do they carry all the way over into the next feeding?
    >
  - Upon opening the eating scene present pet's hunger
    - With no feed history (fresh start), pet hungers for 1 tomato
  - Allow the player to select how many tomatoes to feed pet
    - Pet can be fed up to its hunger + 1
  - Inform player that all tomatoes not fed will be juiced automatically
  - Confirm button
    - Write the day to NVM
      - Pet's hunger is adjusted
      - Pet's health is adjusted
      - Tomatoes fed removed, remaining juiced
- Pet maintenance
  - Introduce check, runs at midnight
    - Must have save data for last time a day was written to NVM
    - Don't read often, maybe only on 1hz clock
    - Performed outside of scene maintenance
- Pet rework
  - Switch from Ferris to a custom designed pet
