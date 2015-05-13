Untested tarballs for Rust nightly channel + bleeding edge features

# Installation

I recommend using multirust:

https://github.com/brson/multirust

$ cd /path/to/tarball

$ tar xf $TARBALL && rm $TARBALL

$ multirust update rusty-edge --copy-local /path/to/tarball

$ cd rusty-edge/unsized_structs

$ multirust override rusty-edge

$ rustc demo.rs && ./demo

# Disclaimer

THESE BINARIES ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# Source code and licenses

Rust: https://github.com/japaric/rust

Rust is dual licensed under the MIT license and the Apache license (version 2.0), for the exact
terms and conditions check its repository.

# Build scripts

All the bash scripts I use to build these binaries can be found in the following repository:

https://github.com/japaric/rusty-edge
