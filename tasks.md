
### character creation process
---
1) Choose a class
2) Determine origin
    - select background
    - select race
3) Determine ability scores
4) Choose an alignment
5) Fill in details

### on hold
---
- [ ] import all srd data
    - [ ] races
        - [ ] aasimar
        - [ ] dragonborn
        - [ ] dwarf
        - [ ] elf
        - [ ] gnome
        - [ ] goliath
        - [ ] halfling
        - [ ] human
        - [ ] orc
        - [ ] tiefling
    - [ ] background
        - [ ] acolyte
        - [ ] artisan
        - [ ] charlatan
        - [ ] criminal
        - [ ] entertainer
        - [ ] farmer
        - [ ] guard
        - [ ] guide
        - [ ] hermit
        - [ ] merchant
        - [ ] noble
        - [ ] sage
        - [ ] sailor
        - [ ] scribe
        - [ ] soldier
        - [ ] wayfarer
    - [ ] classes
        - [ ] paladin
        - [ ] ...
    - [ ] feats
        - [ ] tough
        - [ ] ...

### on deck
---
- [ ] (1) character struct
- [ ] (1) features struct and loader
    - NOTE: clarify overlap between feature json and json feature field
- [ ] (3) functionality for accessing character attributes from string descriptor
- [ ] (3) choose implementation

### in progress
---
- [ ] rust json importer
    - [x] loader graceful error handling
    - [ ] separate class and subclass loading (maybe, should let subclass: Class;?)
- [ ] build tui minimum viable product
    - [x] stat selector
        - [x] roller
        - [x] select one stat (in main())
        - [x] iterate over stats to select
        - [x] pretty format
    - [ ] background selector

### complete
---
- [x] json data containers
    - [x] add "parent" field handling for subclasses
    - [x] create a DiceRef struct for parsing json roll obj and building DiceSpec
        - need to decide between this or a comprehensive DiceSpec object
        - pros of DiceRef is keeping DiceSpec simple and dynamically generating DiceSpec per reference when required. DiceSpec can change, DiceRef is always the same.
        - cons, confusion from having two Dice objects, one roll object.
- [x] consider moving Database to new data.rs file
    - [x] add an "add" function to impl for new content
- [x] load background as object successfully
- [x] test load each type of object
    - [x] class
    - [x] subclass
    - [x] feats
    - [x] races
- [x] full importer, saved to Vec of all data
    - [x] complete file iterator
    - [x] category checker
    - [x] list generate and save


### archive
---


