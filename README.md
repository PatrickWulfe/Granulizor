# Granulizor

Granular Synthesis VST built in Rust

- What is granular synthesis?

Short answer:  It is taking "grains", very short snippets of audio from a file, and repeating them to create a new sound.

Long answer: https://en.wikipedia.org/wiki/Granular_synthesis


- Main features of the synth:
1) Grain size: This parameter controls the size of the grain in miliseconds.
2) Sample start: This parameter controls where in the sample the synth will take the loop from.
3) Pitched mode: When turned on, the synth will use linear interpolation to repitch the sample to whatever note is being played
   in the midi information being passed to the synth.
4) Sample select: Currently, GUIs have not been implimented in the Rust VST API, so the only way I could add some form of sample 
   selector was to have it be a parameter that let you cycle through different samples.  I included 2 samples to cycle through
   as a proof of concept.

   
- Setting up
1) Due to the fact that the find_folder crate doesn't work with VSTs (I believe this has to do with how the DLLs are loaded),
   and the lack of ability to create any form of gui, the path to the "assets" folder must be hard coded into the source before
   compiling.
2) I use Bitwig as my VST host (or DAW), but any you can find for your operating system should work.
3) Once your host or DAW is up and running and you have built the DLL, you should be able to drag and drop the DLL into the DAW
   and it should make a new channel.  You can then drag and drop the "midi.mid" file from the "assets" folder onto that channel.  You should now be able
   to hit play and hear the initial sounds created by the VST.
4) Now you should be able to adjust the parameters and play with them to create sounds based on the samples in the "assets" folder.