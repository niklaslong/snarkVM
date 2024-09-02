# Fuzzing programs (instructions) with AFL++ and a custom grammar mutator

A few things you'll need in your local environment (in addition to this snarkVM branch) to get started:

```sh
# Install cargo-afl with cargo.
cargo install cargo-afl
# Clone the grammar mutator repository.
git clone https://github.com/AFLplusplus/Grammar-Mutator.git
```

You'll also need to install the grammar mutator dependencies you're missing. If you're on Linux the mutator's README should guide you in the right direction.

```sh
brew install openjdk 
# You might also need to set JAVA_HOME.
export JAVA_HOME=$(/usr/libexec/java_home)
```

On macOS you'll need to tweak the Makefile, this patch worked for me.

```
diff --git a/src/Makefile b/src/Makefile
index 43a255f..d0fc8cd 100644
--- a/src/Makefile
+++ b/src/Makefile
@@ -69,7 +69,7 @@ endif
 all: $(TARGETS)
 
 $(GRAMMAR_MUTATOR_LIB): $(LIB_OBJS)
-       $(CXX) -fPIC $(C_FLAGS) -shared -Wl,-soname,$(GRAMMAR_MUTATOR_LIB) -o $@ $^ $(LDFLAGS)
+       $(CXX) -fPIC $(C_FLAGS) -shared -Wl,-install_name,$(GRAMMAR_MUTATOR_LIB) -o $@ $^ $(LDFLAGS)
 
 %.o: %.c
        $(CC) $(C_DEFINES) $(C_INCLUDES) -fPIC $(C_FLAGS) -o $@ -c $<
```

Next up, building the grammar mutator. If all goes well, you should have a `libgrammarmutator-aleo.so` file as output.

```sh
# In Grammar-Mutator/.
make GRAMMAR_FILE=../snarkVM/afl/grammars/aleo.json
```

Now, back to snarkVM.

```sh
# In snarkVM/afl/.
# This one points AFL to the custom mutator library we've just built.
export AFL_CUSTOM_MUTATOR_LIBRARY=../../Grammar-Mutator/libgrammarmutator-aleo.so

# Specify that only the grammar mutator should be used (omit the default bit flips etc).
export AFL_CUSTOM_MUTATOR_ONLY=1      

# And now we can build!
cargo afl build --release

# AFL might enforce that you perform several performance-sensitive tweaks; there is a dedicated script that does this:
cargo-afl afl system-config

# If the build was succesful, you can now fuzz; you may want to tweak the timeout duration in ms (-t) and the input size range (-g and -G).
cargo afl fuzz -t 4000 -i seeds -o out -g 24 -G 2048 target/release/afl 
```

Results will be saved to the `out` directory. I've included a handy tool (`run_all.sh`), which will execute each of the crashes in order and print the errors to stdout. 

## Divergences in ABNF vs JSON:

- `call` should have multiple register accesses, currently there's only 1
- `cws` isn't implemented, replaced with `ws` instead (aka no support for comments)
