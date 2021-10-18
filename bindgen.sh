#!/bin/bash
bindgen --dynamic-loading wintun wintun/wintun-wrapper2.h > src/wintun_raw.rs
