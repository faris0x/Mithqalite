#!/bin/bash
echo "Step 1. Checking for cargo-dylint and dylint-link (cargo install cargo-dylint dylint-link)" &&
command -v cargo-dylint > /dev/null &&
command -v dylint-link > /dev/null &&
echo "Step 2. Checking for updates" &&
rustup update &&
echo "Step 3. Updating headers" &&
cd ../fix-headers &&
cargo test --release &&
cd ../malachite/mithqalite-base &&
echo "Step 4. Formatting mithqalite-base" &&
bash ../superfmt.sh &&
echo "Step 5. Checking mithqalite-base lib" &&
cargo check --lib &&
echo "Step 6. Checking mithqalite-base lib with random" &&
cargo check --lib --features random &&
echo "Step 7. Checking mithqalite-base lib with enable_serde" &&
cargo check --lib --features enable_serde &&
echo "Step 8. Checking mithqalite-base lib with enable_serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 9. Checking all mithqalite-base targets with bin_build" &&
cargo check --all-targets --features bin_build &&
echo "Step 10. Checking mithqalite-base lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 11. Checking mithqalite-base lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 12. Checking mithqalite-base lib with enable_serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 13. Checking mithqalite-base lib with enable_serde, random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 14. Checking all mithqalite-base targets with bin_build and no std" &&
cargo check --all-targets --features bin_build --no-default-features &&
cd ../mithqalite-nz &&
echo "Step 15. Formatting mithqalite-nz" &&
bash ../superfmt.sh &&
echo "Step 16. Checking mithqalite-nz lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 17. Checking mithqalite-nz lib" &&
cargo check --lib &&
echo "Step 18. Checking mithqalite-nz lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 19. Checking mithqalite-nz lib with random" &&
cargo check --lib --features random &&
echo "Step 20. Checking mithqalite-nz lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 21. Checking mithqalite-nz lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 22. Checking mithqalite-nz lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 23. Checking mithqalite-nz lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 24. Checking mithqalite-nz lib with pyo3 and the limited Python API" &&
cargo check --lib --features enable_pyo3 --features pyo3/abi3-py39 &&
echo "Step 25. Checking mithqalite-nz lib with 32_bit_limbs, pyo3, and the limited Python API" &&
cargo check --lib --features 32_bit_limbs --features enable_pyo3 --features pyo3/abi3-py39 &&
echo "Step 26. Checking all mithqalite-nz targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 27. Checking all mithqalite-nz targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 28. Checking all mithqalite-nz targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 29. Checking all mithqalite-nz targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 30. Checking mithqalite-nz lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 31. Checking mithqalite-nz lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 32. Checking mithqalite-nz lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 33. Checking mithqalite-nz lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 34. Checking mithqalite-nz lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 35. Checking mithqalite-nz lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 36. Checking mithqalite-nz lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 37. Checking mithqalite-nz lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 38. Checking all mithqalite-nz targets with bin_build, 32_bit_limbs, and serde and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 39. Checking all mithqalite-nz targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 40. Checking all mithqalite-nz targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 41. Checking all mithqalite-nz targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../mithqalite-q &&
echo "Step 42. Formatting mithqalite-q" &&
bash ../superfmt.sh &&
echo "Step 43. Checking mithqalite-q lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 44. Checking mithqalite-q lib" &&
cargo check --lib &&
echo "Step 45. Checking mithqalite-q lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 46. Checking mithqalite-q lib with random" &&
cargo check --lib --features random &&
echo "Step 47. Checking mithqalite-q lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 48. Checking mithqalite-q lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 49. Checking mithqalite-q lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 50. Checking mithqalite-q lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 51. Checking all mithqalite-q targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 52. Checking all mithqalite-q targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 53. Checking all mithqalite-q targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 54. Checking all mithqalite-q targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 55. Checking mithqalite-q lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 56. Checking mithqalite-q lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 57. Checking mithqalite-q lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 58. Checking mithqalite-q lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 59. Checking mithqalite-q lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 60. Checking mithqalite-q lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 61. Checking mithqalite-q lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 62. Checking mithqalite-q lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 63. Checking all mithqalite-q targets with bin_build, 32_bit_limbs, and serde and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 64. Checking all mithqalite-q targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 65. Checking all mithqalite-q targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 66. Checking all mithqalite-q targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../mithqalite-float &&
echo "Step 67. Formatting mithqalite-float" &&
bash ../superfmt.sh &&
echo "Step 68. Checking mithqalite-float lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 69. Checking mithqalite-float lib" &&
cargo check --lib &&
echo "Step 70. Checking mithqalite-float lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 71. Checking mithqalite-float lib with random" &&
cargo check --lib --features random &&
echo "Step 72. Checking mithqalite-float lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 73. Checking mithqalite-float lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 74. Checking mithqalite-float lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 75. Checking mithqalite-float lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 76. Checking all mithqalite-float targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 77. Checking all mithqalite-float targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 78. Checking all mithqalite-float targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 79. Checking all mithqalite-float targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 80. Checking mithqalite-float lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 81. Checking mithqalite-float lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 82. Checking mithqalite-float lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 83. Checking mithqalite-float lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 84. Checking mithqalite-float lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 85. Checking mithqalite-float lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 86. Checking mithqalite-float lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 87. Checking mithqalite-float lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 88. Checking all mithqalite-float targets with bin_build, 32_bit_limbs, and serde no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 89. Checking all mithqalite-float targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 90. Checking all mithqalite-float targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 91. Checking all mithqalite-float targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../mithqalite-criterion-bench &&
echo "Step 92. Formatting mithqalite-criterion-bench" &&
bash ../superfmt.sh &&
echo "Step 93. Checking mithqalite-criterion-bench" &&
cargo check &&
cd ../mithqalite-bigint &&
echo "Step 94. Formatting mithqalite-bigint" &&
bash ../superfmt.sh &&
echo "Step 95. Checking all mithqalite-bigint targets" &&
cargo check --all-targets &&
echo "Step 96. Checking all mithqalite-bigint targets with no std" &&
cargo check --all-targets --no-default-features &&
echo "Step 97. Checking the malachite meta-crate" &&
cd ../malachite &&
cargo check --all-targets &&
echo "Step 98. Checking the malachite meta-crate with serde" &&
cargo check --all-targets --features enable_serde &&
cd .. &&
echo "Step 99. Checking and testing mithqalite-lints" &&
cd mithqalite-lints &&
cargo fmt --check &&
cargo test -- --test-threads 1 &&
cd .. &&
echo "Step 100. Running additional-lints" &&
bash additional-lints.sh &&
cd mithqalite-base &&
echo "Step 101. Updating mithqalite-base" &&
cargo update &&
echo "Step 102. Formatting mithqalite-base" &&
bash ../superfmt.sh &&
echo "Step 103. Running clippy on mithqalite-base" &&
cargo clippy --all-targets --features bin_build &&
echo "Step 104. Running clippy on mithqalite-base with no std" &&
cargo clippy --all-targets --features bin_build --no-default-features &&
echo "Step 105. Testing mithqalite-base" &&
cargo test --release --tests --features bin_build &&
echo "Step 106. Testing mithqalite-base doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 107. Testing mithqalite-base doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 108. Testing mithqalite-base doctests with random, without test_build" &&
bash ../rundoc.sh --features random &&
echo "Step 109. Documenting mithqalite-base" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 110. Building mithqalite-base lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../mithqalite-nz &&
echo "Step 111. Updating mithqalite-nz" &&
cargo update &&
echo "Step 112. Formatting mithqalite-nz" &&
bash ../superfmt.sh &&
echo "Step 113. Running clippy on mithqalite-nz" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 114. Running clippy on mithqalite-nz with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 115. Running clippy on mithqalite-nz with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 116. Running clippy on mithqalite-nz with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 117. Testing mithqalite-nz" &&
cargo test --release --tests --features test_build --features enable_serde &&
echo "Step 118. Testing mithqalite-nz doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 119. Testing mithqalite-nz with 32_bit_limbs" &&
cargo test --release --tests --features test_build --features 32_bit_limbs --features enable_serde &&
echo "Step 120. Testing mithqalite-nz doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs&&
echo "Step 121. Testing mithqalite-nz doctests with 32_bit_limbs and random" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs --features random &&
echo "Step 122. Running extra tests for mithqalite-nz" &&
python3 extra-tests.py &&
echo "Step 123. Documenting mithqalite-nz" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features doc-images --features random &&
echo "Step 124. Building mithqalite-nz lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
echo "Step 125. Building mithqalite-nz lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../mithqalite-q &&
echo "Step 126. Updating mithqalite-q" &&
cargo update &&
echo "Step 127. Formatting mithqalite-q" &&
bash ../superfmt.sh &&
echo "Step 128. Running clippy on mithqalite-q" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 129. Running clippy on mithqalite-q with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 130. Running clippy on mithqalite-q with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 131. Running clippy on mithqalite-q with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 132. Testing mithqalite-q" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 133. Testing mithqalite-q doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 134. Testing mithqalite-q with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 135. Testing mithqalite-q doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 136. Testing mithqalite-q doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 137. Testing mithqalite-q doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 138. Documenting mithqalite-q" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 139. Building mithqalite-q lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 140. Building mithqalite-q lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../mithqalite-float &&
echo "Step 141. Updating mithqalite-float" &&
cargo update &&
echo "Step 142. Formatting mithqalite-float" &&
bash ../superfmt.sh &&
echo "Step 143. Running clippy on mithqalite-float" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 144. Running clippy on mithqalite-float with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 145. Running clippy on mithqalite-float with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 146. Running clippy on mithqalite-float with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 147. Testing mithqalite-float" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 148. Testing mithqalite-float doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 149. Testing mithqalite-float with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features 32_bit_limbs &&
echo "Step 150. Testing mithqalite-float doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 151. Testing mithqalite-float doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 152. Testing mithqalite-float doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 153. Documenting mithqalite-float" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 154. Building mithqalite-float lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 155. Building mithqalite-float lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../mithqalite-bigint &&
echo "Step 156. Updating mithqalite-bigint" &&
cargo update &&
echo "Step 157. Formatting mithqalite-bigint" &&
bash ../superfmt.sh &&
echo "Step 158. Running clippy on mithqalite-bigint" &&
cargo clippy --all-targets &&
echo "Step 159. Running clippy on mithqalite-bigint with no std" &&
cargo clippy --all-targets --no-default-features &&
echo "Step 160. Testing mithqalite-bigint" &&
cargo test --release &&
echo "Step 161. Documenting mithqalite-bigint" &&
cargo doc --lib --no-deps &&
echo "Step 162. Building mithqalite-bigint lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite &&
echo "Step 163. Documenting malachite" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
cd ../mithqalite-criterion-bench &&
echo "Step 164. Updating mithqalite-criterion-bench" &&
cargo update &&
echo "Step 165. Formatting mithqalite-criterion-bench" &&
bash ../superfmt.sh &&
cd .. &&
echo "Step 166. Running additional-lints" &&
bash additional-lints.sh &&
echo "Step 167. Testing against FLINT" &&
cd oracle-test &&
cargo run --release -- flint &&
echo "Step 168. Testing against Azurite" &&
cargo run --release -- azurite &&
cd .. &&
echo "Step 169. Running the FFT's AVX2 kernels under x86_64 emulation" &&
bash x86-fft-check.sh &&
echo "Step 170. Checking links" &&
cd ../check-malachite-links &&
cargo run --release
