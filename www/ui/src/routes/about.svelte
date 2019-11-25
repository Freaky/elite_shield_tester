
<section>
  <h2>About</h2>

  <p>
    This tool allows you to determine the theoretically-best combination of
    shield generator and shield booster engineering for an
    <a target="_top" href="https://www.elitedangerous.com/">Elite Dangerous</a>
    starship.
  </p>

  <p>
    It is based on
    <a target="_top" href="https://www.youtube.com/channel/UCg3QI9rHzPgvR7KTKSCtPHg">Down to Earth Astronomy</a>'s
    <a target="_top" href="https://github.com/DownToEarthAstronomy/D2EA_Shield_tester">shield tester</a>
    PowerShell script, as demonstrated in <a href="https://www.youtube.com/watch?v=lHYpwEU6GpY">this video</a>.
    It gives typically-identical results, but runs in milliseconds instead of minutes
    without compromising on result quality.
  </p>

  <section>
  	<h3>Technical Information</h3>

	  <p>
	    This tool is written in the high-performance buzzword systems programming language,
	    <a target="_top" href="https://www.rust-lang.org/">Rust</a>, with a frontend written in
	    Javascript (using <a target="_top" href="https://sapper.svelte.dev/">Sapper</a>) and a little PHP.
	  </p>

    <p>
      To avoid having to perform a fully exhaustive search, only resistance boosters
      for damage types actually being received are considered, and all remaining
      possible pairs of boosters are filtered using a k-d tree to eliminate combinations
      that will always be beaten by other pairs.
    </p>

	  <p>
	    Source code for both this web frontend and the backend command-line tool is available
	    <a target="_top" href="https://github.com/Freaky/elite_shield_tester">on Github</a>,
	    and contributions and bug reports are welcome.
	  </p>
	</section>

  <section>
    <h3>Acknowledgements</h3>

    <p>
    	This website and the backend tool is written and hosted by
    	<a target="_top" href="https://hur.st/">Thomas Hurst</a>.
   	</p>

    <p>
      Special thanks to:
    </p>

    <ul>
      <li>
        <a target="_top" href="https://www.youtube.com/channel/UCg3QI9rHzPgvR7KTKSCtPHg">Down to Earth Astronomy</a>
        for the original <a target="_top" href="https://github.com/DownToEarthAstronomy/D2EA_Shield_tester">PowerShell implementation</a>
        which inspired this tool.
      </li>
      <li>
        <a target="_top" href="https://github.com/ntt">Jamie "Entity" van den Berge</a> for the
        novel pairwise filtering algorithm which provides some very impressive speedups.
      </li>
      <li>
        <a target="_top" href="https://github.com/Thurion">Sebastian Bauer</a> for his Python port,
        which I used to help generate the data files this tool currently uses.
      </li>
      <li>
        <a target="_top" href="https://github.com/vanderaj">Andrew van der Stock</a> for his Go version
        and bug reports.
      </li>
    </ul>
  </section>
</section>
