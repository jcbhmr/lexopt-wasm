<h1><a id="lexopt_world"></a>World lexopt-world</h1>
<p>Docs for WORLD</p>
<ul>
<li>Exports:
<ul>
<li>interface <a href="#jcbhmr_lexopt_lexopt"><code>jcbhmr:lexopt/lexopt</code></a></li>
</ul>
</li>
</ul>
<h2><a id="jcbhmr_lexopt_lexopt"></a>Export interface jcbhmr:lexopt/lexopt</h2>
<hr />
<h3>Types</h3>
<h4><a id="parser"></a><code>resource parser</code></h4>
<p>Docs for PARSER</p>
<h4><a id="raw_args"></a><code>resource raw-args</code></h4>
<h4><a id="values_iter"></a><code>resource values-iter</code></h4>
<h4><a id="arg"></a><code>variant arg</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="arg.short"></a><code>short</code>: <code>char</code></li>
<li><a id="arg.long"></a><code>long</code>: <code>string</code></li>
<li><a id="arg.value"></a><code>value</code>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="error_missing_value"></a><code>record error-missing-value</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="error_missing_value.option"></a><code>option</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="error_unexpected_value"></a><code>record error-unexpected-value</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="error_unexpected_value.option"></a><code>option</code>: <code>string</code></li>
<li><a id="error_unexpected_value.value"></a><code>value</code>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="error_parsing_failed"></a><code>record error-parsing-failed</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="error_parsing_failed.value"></a><code>value</code>: <code>string</code></li>
<li><a id="error_parsing_failed.error"></a><a href="#error"><code>error</code></a>: <code>string</code></li>
</ul>
<h4><a id="error"></a><code>variant error</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="error.missing_value"></a><code>missing-value</code>: <a href="#error_missing_value"><a href="#error_missing_value"><code>error-missing-value</code></a></a></li>
<li><a id="error.unexpected_option"></a><code>unexpected-option</code>: <code>string</code></li>
<li><a id="error.unexpected_argument"></a><code>unexpected-argument</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="error.unexpected_value"></a><code>unexpected-value</code>: <a href="#error_unexpected_value"><a href="#error_unexpected_value"><code>error-unexpected-value</code></a></a></li>
<li><a id="error.parsing_failed"></a><code>parsing-failed</code>: <a href="#error_parsing_failed"><a href="#error_parsing_failed"><code>error-parsing-failed</code></a></a></li>
<li><a id="error.non_unicode_value"></a><code>non-unicode-value</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="error.custom"></a><code>custom</code>: <code>string</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="method_parser_bin_name"></a><code>[method]parser.bin-name: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_bin_name.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_bin_name.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="static_parser_from_args"></a><code>[static]parser.from-args: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="static_parser_from_args.args"></a><code>args</code>: list&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_parser_from_args.0"></a> own&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h4><a id="static_parser_from_env"></a><code>[static]parser.from-env: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="static_parser_from_env.0"></a> own&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h4><a id="static_parser_from_iter"></a><code>[static]parser.from-iter: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="static_parser_from_iter.iter"></a><code>iter</code>: list&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_parser_from_iter.0"></a> own&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h4><a id="method_parser_next"></a><code>[method]parser.next: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_next.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_next.0"></a> result&lt;option&lt;<a href="#arg"><a href="#arg"><code>arg</code></a></a>&gt;, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_parser_optional_value"></a><code>[method]parser.optional-value: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_optional_value.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_optional_value.0"></a> option&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h4><a id="method_parser_raw_args"></a><code>[method]parser.raw-args: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_raw_args.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_raw_args.0"></a> result&lt;own&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_parser_try_raw_args"></a><code>[method]parser.try-raw-args: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_try_raw_args.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_try_raw_args.0"></a> option&lt;own&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_parser_value"></a><code>[method]parser.value: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_value.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_value.0"></a> result&lt;list&lt;<code>u8</code>&gt;, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_parser_values"></a><code>[method]parser.values: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_values.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_values.0"></a> result&lt;own&lt;<a href="#values_iter"><a href="#values_iter"><code>values-iter</code></a></a>&gt;, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_parser_clone"></a><code>[method]parser.clone: func</code></h4>
<p>Clone trait</p>
<h5>Params</h5>
<ul>
<li><a id="method_parser_clone.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_clone.0"></a> own&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h4><a id="method_parser_clone_from"></a><code>[method]parser.clone-from: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_parser_clone_from.self"></a><code>self</code>: borrow&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
<li><a id="method_parser_clone_from.source"></a><code>source</code>: own&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_parser_clone_from.0"></a> own&lt;<a href="#parser"><a href="#parser"><code>parser</code></a></a>&gt;</li>
</ul>
<h4><a id="method_raw_args_as_slice"></a><code>[method]raw-args.as-slice: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_as_slice.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_as_slice.0"></a> list&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h4><a id="method_raw_args_peek"></a><code>[method]raw-args.peek: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_peek.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_peek.0"></a> option&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h4><a id="method_raw_args_next"></a><code>[method]raw-args.next: func</code></h4>
<p>Iterator trait</p>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_next.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_next.0"></a> option&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h4><a id="method_raw_args_size_hint"></a><code>[method]raw-args.size-hint: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_size_hint.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_size_hint.0"></a> (<code>u32</code>, option&lt;<code>u32</code>&gt;)</li>
</ul>
<h4><a id="method_raw_args_count"></a><code>[method]raw-args.count: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_count.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_count.0"></a> <code>u32</code></li>
</ul>
<h4><a id="method_raw_args_last"></a><code>[method]raw-args.last: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_last.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_last.0"></a> option&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h4><a id="method_raw_args_nth"></a><code>[method]raw-args.nth: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_raw_args_nth.self"></a><code>self</code>: borrow&lt;<a href="#raw_args"><a href="#raw_args"><code>raw-args</code></a></a>&gt;</li>
<li><a id="method_raw_args_nth.n"></a><code>n</code>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_raw_args_nth.0"></a> option&lt;list&lt;<code>u8</code>&gt;&gt;</li>
</ul>
<h4><a id="arg_unexpected"></a><code>arg-unexpected: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="arg_unexpected.self"></a><code>self</code>: <a href="#arg"><a href="#arg"><code>arg</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="arg_unexpected.0"></a> <a href="#error"><a href="#error"><code>error</code></a></a></li>
</ul>
