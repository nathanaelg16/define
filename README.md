# DEFINE

Look up word definitions straight from your command line!  

Powered by [Wordnik](https://wordnik.com) and written entirely in Rust.

![define](https://github.com/nathanaelg16/define/assets/12057936/1c88aa86-f118-4215-bdc6-5ba13f307131)


## Installation
1. Clone the repo and install using `cargo`.
    ```bash
    git clone "https://github.com/nathanaelg16/define.git"
    cd define
    cargo install --path .
    ```
1. Obtain a [free API key](https://developer.wordnik.com/) from Wordnik.
1. Run the command and paste the API key when prompted, or supply it via the -apiKey flag, i.e. `define -apiKey <api_key>`.
   
   ![Screenshot_20240531_135320](https://github.com/nathanaelg16/define/assets/12057936/53745ea8-54d3-4d55-a5f3-a8dc7a34548b)


## Usage

```
define <word> [OPTIONS]

Options:
    -D --dictionary --dictionaries  [...]    Source dictionaries to return definitions from, separated by a space
    -S --partOfSpeech               [...]    The part of speech of the word whose definition is requested
    -L --limit                      [...]    Maximum number of results to return
    -A --audio                               Request an audio pronunciation of the word
    -R --includeRelated                      Request related words with definitions
    -C --useCanonical                        Tries to return the correct word root (e.g. 'cats' -> 'cat')
    -X --examples                            Request examples for the word
    -F --frequency                  [...]    Request word usage over time
    -H --hyphenation                         Request syllable information for the word
    -P --pronunciation              [...]    Request text pronunciation for the word with the specified pronunciation type
    -T --thesaurus                           Request synonym and antonym information for the word
    -U --usage --help                        Display this usage guide
```
