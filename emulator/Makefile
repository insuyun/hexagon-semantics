# From Sim_API_Example
ARCH=62
Q6VERSION = v$(ARCH)

SDK_VERSION ?= 3.5.2
HEXAGON_TOOLS_VERSION ?= 8.3.07

SDK_ROOT ?= ${HOME}/Qualcomm
HEXAGON_SDK_ROOT ?= ${SDK_ROOT}/Hexagon_SDK/${SDK_VERSION}
HEXAGON_TOOLS_ROOT ?= ${HEXAGON_SDK_ROOT}/tools/HEXAGON_Tools/${HEXAGON_TOOLS_VERSION}

CC_HEXAGON := ${HEXAGON_TOOLS_ROOT}/Tools/bin/hexagon-clang
CXX_HEXAGON := ${HEXAGON_TOOLS_ROOT}/Tools/bin/hexagon-clang++

SRC_TOP := $(shell pwd)
SYSTEM_NAME = hexagon-system
TARGET := main
TEST_DIR := "$(SRC_TOP)/src"
BIN_DIR := "$(SRC_TOP)/bin"

RELEASE_DIR := ${HEXAGON_TOOLS_ROOT}/Tools
INCLUDE_DIR := ${RELEASE_DIR}/include/iss
LIB_DIR := ${RELEASE_DIR}/lib

# Compiler to use
# g++ must be g++-4.1.2 or later. Please set LD_LIBRARY_PATH to point to
# the g++ distribution lib dir (typically $(g++-4.1.2-or-later-install-dir)/lib64
# for linux64 and $(g++-4.1.2-or-later-ins all-dir)/lib for linux32)
CC_SYSTEM := g++
CFLAGS_SYSTEM = -c -fPIC -Wno-write-strings -fomit-frame-pointer -ffast-math
CFLAGS_SYSTEM += -I./ -Wall -g -I$(INCLUDE_DIR) -o $(BIN_DIR)/$(SYSTEM_NAME).obj
LFLAGS_SYSTEM = -L$(LIB_DIR) -L$(LIB_DIR)/iss -l:libhexagoniss$(Q6VERSION).so -lc++
SIMFLAGS =  -m$(Q6VERSION) -G 0 $(BIN_DIR)/$(TARGET).elf
CFLAGS = -m$(Q6VERSION) -g


all: clean build

clean:
	rm -rf $(BIN_DIR) pmu_statsfile.txt stats.txt

build:
	mkdir -p $(BIN_DIR)
	$(CC_SYSTEM) $(CFLAGS_SYSTEM) $(TEST_DIR)/$(SYSTEM_NAME).cpp
	$(CC_SYSTEM) -o $(BIN_DIR)/$(SYSTEM_NAME) $(BIN_DIR)/$(SYSTEM_NAME).obj $(LFLAGS_SYSTEM)
	$(CC_HEXAGON) $(CFLAGS) -o $(BIN_DIR)/$(TARGET).elf $(TEST_DIR)/$(TARGET).c
	chmod +x $(BIN_DIR)/$(SYSTEM_NAME)

sim:
	LD_LIBRARY_PATH=$(LIB_DIR):$(LIB_DIR)/iss \
	$(BIN_DIR)/$(SYSTEM_NAME) $(SIMFLAGS) $(reg_input)


