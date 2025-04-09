.PHONY: all clean setup build parsers test bench install

PROJECT_ROOT = ..
RUST_SRC = $(PROJECT_ROOT)/src-tauri/src
HASKELL_SRC = src
CBITS = cbits
BUILD = dist
PARSER_SRC = $(HASKELL_SRC)/Parser

# Default target
all: build

# Setup environment
setup:
    tools/setup_parsers.bat
    cabal update

# Generate lexers and parsers
parsers:
    alex $(PARSER_SRC)/CompletionRulesLexer.x -o $(PARSER_SRC)/CompletionRulesLexer.hs
    happy $(PARSER_SRC)/CompletionRulesParser.y -o $(PARSER_SRC)/CompletionRulesParser.hs
    alex $(PARSER_SRC)/QueryLexer.x -o $(PARSER_SRC)/QueryLexer.hs
    happy $(PARSER_SRC)/QueryParser.y -o $(PARSER_SRC)/QueryParser.hs

# Clean build artifacts
clean:
    cabal clean
    rm -rf $(BUILD)
    rm -f $(PARSER_SRC)/*Lexer.hs $(PARSER_SRC)/*Parser.hs

# Build Haskell components
build: parsers
    cabal build --ghc-options="-O2 -fllvm -optlo-O3"
    mkdir -p $(BUILD)/lib $(BUILD)/include $(BUILD)/bin
    cp `cabal list-bin lms-haskell-analyzer` $(BUILD)/bin/
    cp `find dist-newstyle -name "*.a"` $(BUILD)/lib/
    cp -r $(CBITS)/*.h $(BUILD)/include/

# Run tests
test: parsers
    cabal test

# Run benchmarks
bench: parsers
    cabal bench

# Install into main project
install: build
    mkdir -p $(PROJECT_ROOT)/src-tauri/lib $(PROJECT_ROOT)/src-tauri/include $(PROJECT_ROOT)/src-tauri/bin
    cp -r $(BUILD)/lib/*.a $(PROJECT_ROOT)/src-tauri/lib/
    cp -r $(BUILD)/include/*.h $(PROJECT_ROOT)/src-tauri/include/
    cp -r $(BUILD)/bin/* $(PROJECT_ROOT)/src-tauri/bin/