# This file was autogenerated by some hot garbage in the `uniffi` crate.
# Trust me, you don't want to mess with it!

# Tell mypy (a type checker) to ignore all errors from this file.
# See https://mypy.readthedocs.io/en/stable/config_file.html?highlight=ignore-errors#confval-ignore_errors
# mypy: ignore-errors

# Common helper code.
#
# Ideally this would live in a separate .py file where it can be unittested etc
# in isolation, and perhaps even published as a re-useable package.
#
# However, it's important that the details of how this helper code works (e.g. the
# way that different builtin types are passed across the FFI) exactly match what's
# expected by the rust code on the other side of the interface. In practice right
# now that means coming from the exact some version of `uniffi` that was used to
# compile the rust component. The easiest way to ensure this is to bundle the Python
# helpers directly inline like we're doing here.

import os
import sys
import ctypes
import enum
import struct
import contextlib
import datetime
{%- if ci.has_async_fns() %}
import asyncio
{%- endif %}
import contextvars
import enum
{%- for req in self.imports() %}
{{ req.render() }}
{%- endfor %}

# Used for default argument values
DEFAULT = object()

{% include "RustBufferTemplate.py" %}
{% include "Helpers.py" %}
{% include "RustBufferHelper.py" %}
{%- if ci.has_async_fns() %}
{% include "RustFutureTemplate.py" %}
{%- endif %}

# Contains loading, initialization code,
# and the FFI Function declarations in a com.sun.jna.Library.
{% include "NamespaceLibraryTemplate.py" %}

# Public interface members begin here.
{{ type_helper_code }}

{%- for func in ci.function_definitions() %}
{%- include "TopLevelFunctionTemplate.py" %}
{%- endfor %}

__all__ = [
    "InternalError",
    {%- for e in ci.enum_definitions() %}
    "{{ e|type_name }}",
    {%- endfor %}
    {%- for record in ci.record_definitions() %}
    "{{ record|type_name }}",
    {%- endfor %}
    {%- for func in ci.function_definitions() %}
    "{{ func.name()|fn_name }}",
    {%- endfor %}
    {%- for obj in ci.object_definitions() %}
    "{{ obj|type_name }}",
    {%- endfor %}
    {%- for e in ci.error_definitions() %}
    "{{ e|type_name }}",
    {%- endfor %}
    {%- for c in ci.callback_interface_definitions() %}
    "{{ c.name()|class_name }}",
    {%- endfor %}
]

{% import "macros.py" as py %}
