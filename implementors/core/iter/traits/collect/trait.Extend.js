(function() {var implementors = {
"arrayvec":[["impl&lt;T, const CAP:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;T, CAP&gt;"]],
"either":[["impl&lt;L, R, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt; for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt;,</span>"]],
"hashbrown":[["impl&lt;K, V, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/core/primitive.tuple.html\">(K, V)</a>&gt; for <a class=\"struct\" href=\"hashbrown/hash_map/struct.HashMap.html\" title=\"struct hashbrown::hash_map::HashMap\">HashMap</a>&lt;K, V, S, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Allocator + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;'a, K, V, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/core/primitive.reference.html\">&amp;'a </a>K, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/core/primitive.reference.html\">&amp;'a </a>V)&gt; for <a class=\"struct\" href=\"hashbrown/hash_map/struct.HashMap.html\" title=\"struct hashbrown::hash_map::HashMap\">HashMap</a>&lt;K, V, S, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Allocator + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;'a, K, V, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/core/primitive.tuple.html\">(K, V)</a>&gt; for <a class=\"struct\" href=\"hashbrown/hash_map/struct.HashMap.html\" title=\"struct hashbrown::hash_map::HashMap\">HashMap</a>&lt;K, V, S, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Allocator + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Allocator + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;'a, T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/core/primitive.reference.html\">&amp;'a </a>T&gt; for <a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Allocator + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"indexmap":[["impl&lt;K, V, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.tuple.html\">(K, V)</a>&gt; for <a class=\"struct\" href=\"indexmap/map/struct.IndexMap.html\" title=\"struct indexmap::map::IndexMap\">IndexMap</a>&lt;K, V, S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;'a, K, V, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.reference.html\">&amp;'a </a>K, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.reference.html\">&amp;'a </a>V)&gt; for <a class=\"struct\" href=\"indexmap/map/struct.IndexMap.html\" title=\"struct indexmap::map::IndexMap\">IndexMap</a>&lt;K, V, S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;'a, T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.reference.html\">&amp;'a </a>T&gt; for <a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"]],
"kurbo":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"enum\" href=\"kurbo/enum.PathEl.html\" title=\"enum kurbo::PathEl\">PathEl</a>&gt; for <a class=\"struct\" href=\"kurbo/struct.BezPath.html\" title=\"struct kurbo::BezPath\">BezPath</a>"]],
"plist":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/1.65.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"plist/enum.Value.html\" title=\"enum plist::Value\">Value</a>)&gt; for <a class=\"struct\" href=\"plist/dictionary/struct.Dictionary.html\" title=\"struct plist::dictionary::Dictionary\">Dictionary</a>"]],
"png":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>&gt; for <a class=\"struct\" href=\"png/struct.Transformations.html\" title=\"struct png::Transformations\">Transformations</a>"]],
"proc_macro2":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"enum\" href=\"proc_macro2/enum.TokenTree.html\" title=\"enum proc_macro2::TokenTree\">TokenTree</a>&gt; for <a class=\"struct\" href=\"proc_macro2/struct.TokenStream.html\" title=\"struct proc_macro2::TokenStream\">TokenStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"proc_macro2/struct.TokenStream.html\" title=\"struct proc_macro2::TokenStream\">TokenStream</a>&gt; for <a class=\"struct\" href=\"proc_macro2/struct.TokenStream.html\" title=\"struct proc_macro2::TokenStream\">TokenStream</a>"]],
"skia_safe":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/canvas/struct.SaveLayerFlags.html\" title=\"struct skia_safe::canvas::SaveLayerFlags\">SaveLayerFlags</a>&gt; for <a class=\"struct\" href=\"skia_safe/canvas/struct.SaveLayerFlags.html\" title=\"struct skia_safe::canvas::SaveLayerFlags\">SaveLayerFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/struct.ColorChannelFlag.html\" title=\"struct skia_safe::ColorChannelFlag\">ColorChannelFlag</a>&gt; for <a class=\"struct\" href=\"skia_safe/struct.ColorChannelFlag.html\" title=\"struct skia_safe::ColorChannelFlag\">ColorChannelFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/contour_measure/struct.MatrixFlags.html\" title=\"struct skia_safe::contour_measure::MatrixFlags\">MatrixFlags</a>&gt; for <a class=\"struct\" href=\"skia_safe/contour_measure/struct.MatrixFlags.html\" title=\"struct skia_safe::contour_measure::MatrixFlags\">MatrixFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/font_metrics/struct.Flags.html\" title=\"struct skia_safe::font_metrics::Flags\">Flags</a>&gt; for <a class=\"struct\" href=\"skia_safe/font_metrics/struct.Flags.html\" title=\"struct skia_safe::font_metrics::Flags\">Flags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/matrix/struct.TypeMask.html\" title=\"struct skia_safe::matrix::TypeMask\">TypeMask</a>&gt; for <a class=\"struct\" href=\"skia_safe/matrix/struct.TypeMask.html\" title=\"struct skia_safe::matrix::TypeMask\">TypeMask</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/path_measure/struct.MatrixFlags.html\" title=\"struct skia_safe::path_measure::MatrixFlags\">MatrixFlags</a>&gt; for <a class=\"struct\" href=\"skia_safe/path_measure/struct.MatrixFlags.html\" title=\"struct skia_safe::path_measure::MatrixFlags\">MatrixFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/path/struct.SegmentMask.html\" title=\"struct skia_safe::path::SegmentMask\">PathSegmentMask</a>&gt; for <a class=\"struct\" href=\"skia_safe/path/struct.SegmentMask.html\" title=\"struct skia_safe::path::SegmentMask\">PathSegmentMask</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/struct.SurfacePropsFlags.html\" title=\"struct skia_safe::SurfacePropsFlags\">SurfacePropsFlags</a>&gt; for <a class=\"struct\" href=\"skia_safe/struct.SurfacePropsFlags.html\" title=\"struct skia_safe::SurfacePropsFlags\">SurfacePropsFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/vertices/struct.BuilderFlags.html\" title=\"struct skia_safe::vertices::BuilderFlags\">BuilderFlags</a>&gt; for <a class=\"struct\" href=\"skia_safe/vertices/struct.BuilderFlags.html\" title=\"struct skia_safe::vertices::BuilderFlags\">BuilderFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/gradient_shader/struct.Flags.html\" title=\"struct skia_safe::gradient_shader::Flags\">Flags</a>&gt; for <a class=\"struct\" href=\"skia_safe/gradient_shader/struct.Flags.html\" title=\"struct skia_safe::gradient_shader::Flags\">Flags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/runtime_effect/uniform/struct.Flags.html\" title=\"struct skia_safe::runtime_effect::uniform::Flags\">Flags</a>&gt; for <a class=\"struct\" href=\"skia_safe/runtime_effect/uniform/struct.Flags.html\" title=\"struct skia_safe::runtime_effect::uniform::Flags\">Flags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/svg/canvas/struct.Flags.html\" title=\"struct skia_safe::svg::canvas::Flags\">Flags</a>&gt; for <a class=\"struct\" href=\"skia_safe/svg/canvas/struct.Flags.html\" title=\"struct skia_safe::svg::canvas::Flags\">Flags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"skia_safe/utils/shadow_utils/struct.ShadowFlags.html\" title=\"struct skia_safe::utils::shadow_utils::ShadowFlags\">ShadowFlags</a>&gt; for <a class=\"struct\" href=\"skia_safe/utils/shadow_utils/struct.ShadowFlags.html\" title=\"struct skia_safe::utils::shadow_utils::ShadowFlags\">ShadowFlags</a>"]],
"smallvec":[["impl&lt;A:&nbsp;<a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;&lt;A as <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt;::<a class=\"associatedtype\" href=\"smallvec/trait.Array.html#associatedtype.Item\" title=\"type smallvec::Array::Item\">Item</a>&gt; for <a class=\"struct\" href=\"smallvec/struct.SmallVec.html\" title=\"struct smallvec::SmallVec\">SmallVec</a>&lt;A&gt;"]],
"syn":[["impl&lt;T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt; for <a class=\"struct\" href=\"syn/punctuated/struct.Punctuated.html\" title=\"struct syn::punctuated::Punctuated\">Punctuated</a>&lt;T, P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"],["impl&lt;T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"enum\" href=\"syn/punctuated/enum.Pair.html\" title=\"enum syn::punctuated::Pair\">Pair</a>&lt;T, P&gt;&gt; for <a class=\"struct\" href=\"syn/punctuated/struct.Punctuated.html\" title=\"struct syn::punctuated::Punctuated\">Punctuated</a>&lt;T, P&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"syn/parse/struct.Error.html\" title=\"struct syn::parse::Error\">Error</a>&gt; for <a class=\"struct\" href=\"syn/parse/struct.Error.html\" title=\"struct syn::parse::Error\">Error</a>"]],
"trees":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"struct\" href=\"trees/tree/struct.Tree.html\" title=\"struct trees::tree::Tree\">Tree</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"trees/rc/enum.RcNode.html\" title=\"enum trees::rc::RcNode\">RcNode</a>&lt;T&gt;"]],
"xml":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.str.html\">str</a>, &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.str.html\">str</a>)&gt; for <a class=\"struct\" href=\"xml/namespace/struct.Namespace.html\" title=\"struct xml::namespace::Namespace\">Namespace</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.str.html\">str</a>, &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.str.html\">str</a>)&gt; for <a class=\"struct\" href=\"xml/namespace/struct.NamespaceStack.html\" title=\"struct xml::namespace::NamespaceStack\">NamespaceStack</a>"],["impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;(&amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.str.html\">str</a>, &amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.str.html\">str</a>)&gt; for <a class=\"struct\" href=\"xml/namespace/struct.CheckedTarget.html\" title=\"struct xml::namespace::CheckedTarget\">CheckedTarget</a>&lt;'a&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()