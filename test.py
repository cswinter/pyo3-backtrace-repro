try:
    from cffi import FFI
except ImportError:
    print("pip install cffi, included with PyPy")

ffi = FFI()
lib = ffi.dlopen("./target/debug/deps/libpyo3_backtrace_repro.dylib")

print(lib)
# <cffi.api.FFILibrary_./libtreble.dylib object at 0x107f440d0>

ffi.cdef('void raw_oh_noes();')

print(lib.raw_oh_noes())
