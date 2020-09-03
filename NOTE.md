Reference
---------
# Instruction semantics
- A Complete Formal Semantics of x86-64 User-Level Instruction Set Architecture
    - http://fsl.cs.illinois.edu/index.php/A_Complete_Formal_Semantics_of_x86-64_User-Level_Instruction_Set_Architecture
    - Semantics Github: https://github.com/kframework/X86-64-semantics
    - Related/Used Projects
        - K Framework `[55, 59]` http://www.kframework.org/index.php/Main_Page 
        - Strata Project `[37]` https://stefanheule.com/s/projects/strata/
        - Stoke `[58]` - Described below

- Stratified Synthesis: Automatically Learning the x86-64 Instruction Set
    - http://stoke.stanford.edu/
    - GitHub link: https://github.com/StanfordPL/stoke 
- Lifting Assembly to Intermediate Representation: A Novel Approach Leveraging Compilers
- Stochastic Superoptimization

# Hexagon
- https://github.com/gsmk/hexagon
- Hexagon manual + SDK: https://developer.qualcomm.com/software/hexagon-dsp-sdk/tools

# Analyzing Qualcomm DSP
- Pwn2Own Qualcomm Compute DSP for Fun and Profit (DEFCON '20)
- Attacking Hexagon: Security Analysis of Qualcomm's aDSP (Recon '19)
- Exploiting Qualcomm WLAN And Modem Over-The-Air (BLACKHAT USA '19)
- A Journey Into Hexagon Dissecting a Qualcomm Baseband (DEFCON '18)
- Exploring Qualcomm Baseband via ModKit (CanSecWest '18)
- Baseband exploitation in 2013: Hexagon challenges (ParSec '13)
- Baseband Attacks: Remote Exploitation of Memory Corruptions in Cellular Protocol Stacks (WOOT '12)
- Reverse engineering a Qualcomm baseband (CCC '11)

# Ghidra
- https://ghidra.re/courses/languages/html/sleigh.html
- Implementing a New CPU Architecture for Ghidra
- https://www.reddit.com/r/ghidra/comments/bhhrt0/quick_guide_to_creating_a_processor_in_ghidra/
- Specifying Representations of Machine Instructions
- https://www.reddit.com/r/ghidra/comments/f5lk42/my_experience_writing_processor_modules/
- https://spinsel.dev/


Questions
----------
- How to represent semantics for Hexagon?
    - symbolic expression
    - K framework
        - "A Complete Formal Semantics of x86-64 User-Level Instruction Set Architecture" uses K Framework, while Strata Project
        and Stoke use SMT formulas. They translated Strata's semantics into K framework. why?

- How to construct semantics?
    - automated: strata? compiler? maybe qemu?
    - manual

- How to test semantics (dynamic)?
    - qemu?
    - hardware? is it availalble?

- Can we systematically convert formal semantics into sleigh?
