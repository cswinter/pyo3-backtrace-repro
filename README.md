This repository contains a minimal reproduction of pip packages generated with PyO3 0.9.0-alpha.1 or 0.8.3 and maturin to show filenames and line numbers in backtraces on panic.

Example output:

```
$ maturin build --rustc-extra-args="-C debuginfo=2"
...
$ pip uninstall pyo3-backtrace-repro
...
$ pip install target/wheels/pyo3_backtrace_repro-0.1.0-cp38-cp38-macosx_10_7_x86_64.whl
...
$ RUST_BACKTRACE=1 python
Python 3.8.0 (default, Nov  6 2019, 15:49:01)
[Clang 4.0.1 (tags/RELEASE_401/final)] :: Anaconda, Inc. on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> RUST_BACKTRACE=1 python
>>> from pyo3_backtrace_repro import oh_noes
>>> oh_noes()
thread '<unnamed>' panicked at 'panic', src/lib.rs:10:5
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: pyo3_backtrace_repro::foo
   8: pyo3_backtrace_repro::oh_noes
   9: pyo3_backtrace_repro::__pyo3_get_function_oh_noes::__wrap
  10: cfunction_call_varargs
  11: _PyObject_MakeTpCall
  12: call_function
  13: _PyEval_EvalFrameDefault
  14: _PyEval_EvalCodeWithName
  15: PyRun_InteractiveOneObjectEx
  16: PyRun_InteractiveLoopFlags
  17: PyRun_AnyFileExFlags
  18: Py_RunMain
  19: pymain_main
  20: main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
fatal runtime error: failed to initiate panic, error 5
[1]    80860 abort      RUST_BACKTRACE=1 python
```

