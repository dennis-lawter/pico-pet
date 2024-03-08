# TODO
This list is not final. It will grow and evolve.

- [ ] Intro Scene
  - [ ] Only loaded if NVM is not initialized properly and must be reset
  - [ ] Introduce pet
  - [ ] Explain the pomodoro method
  - [ ] Explain how feeding works
  - [ ] Force the player to save each of the setting components
  - [ ] Finalize by saving and pet greeting scene
- [ ] Pomodoro Scene
  - [ ] Reward player with tomato
  - [ ] Reward player with tomato juice for pomo chains
  - [ ] Reward player with raspberry for cycle completion
- [ ] Eating Scene
  - [ ] Upon opening the eating scene present pet's hunger
    - [ ] With no feed history (fresh start), pet hungers for 1 tomato
  - [ ] Allow the player to select how many tomatoes to feed pet
    - [ ] Pet can be fed up to its hunger + 1
  - [ ] Inform player that all tomatoes not fed will be juiced automatically
  - [ ] Confirm button
    - [ ] Write the day to NVM
      - [ ] Pet's hunger is adjusted
      - [ ] Pet's health is adjusted
      - [ ] Tomatoes fed removed, remaining juiced
- [ ] Pet maintenance
  - [ ] Introduce check, runs at midnight
    - [ ] Must have save data for last time a day was written to NVM
    - [ ] Don't read often, maybe only on 1hz clock
    - [ ] Performed outside of scene maintenance
