(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl Borrow&lt;[u8]&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl Borrow&lt;[u8]&gt; for BytesMut","synthetic":false,"types":[]}];
implementors["bytestring"] = [{"text":"impl Borrow&lt;str&gt; for ByteString","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; Borrow&lt;[T]&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl Borrow&lt;str&gt; for HeaderName","synthetic":false,"types":[]}];
implementors["multipart"] = [{"text":"impl&lt;R&gt; Borrow&lt;R&gt; for Multipart&lt;R&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'s, T&gt; Borrow&lt;[T]&gt; for SliceVec&lt;'s, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["trust_dns_proto"] = [{"text":"impl Borrow&lt;[u8]&gt; for Label","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()