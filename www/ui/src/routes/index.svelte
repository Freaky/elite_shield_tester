<script>
  import Range from '../components/Range.svelte';

  const BaseURL = "/shieldtester/";

  const Ships = {
    "Adder": { "boosters": 2, "shields": [ 1, 3 ] },
    "Alliance Challenger": { "boosters": 4, "shields": [ 4, 6 ] },
    "Alliance Chieftain": { "boosters": 4, "shields": [ 3, 6 ] },
    "Alliance Crusader": { "boosters": 4, "shields": [ 4, 6 ] },
    "Anaconda": { "boosters": 8, "shields": [ 3, 7 ] },
    "Asp Explorer": { "boosters": 4, "shields": [ 3, 6 ] },
    "Asp Scout": { "boosters": 2, "shields": [ 3, 5 ] },
    "Beluga Liner": { "boosters": 6, "shields": [ 5, 6 ] },
    "Cobra Mk III": { "boosters": 2, "shields": [ 3, 4 ] },
    "Cobra Mk IV": { "boosters": 2, "shields": [ 3, 4 ] },
    "Diamondback Explorer": { "boosters": 4, "shields": [ 3, 4 ] },
    "Diamondback Scout": { "boosters": 4, "shields": [ 3, 3 ] },
    "Dolphin": { "boosters": 3, "shields": [ 3, 5 ] },
    "Eagle": { "boosters": 1, "shields": [ 1, 3 ] },
    "Federal Assault Ship": { "boosters": 4, "shields": [ 4, 5 ] },
    "Federal Corvette": { "boosters": 8, "shields": [ 5, 7 ] },
    "Federal Dropship": { "boosters": 4, "shields": [ 4, 6 ] },
    "Federal Gunship": { "boosters": 4, "shields": [ 4, 6 ] },
    "Fer-de-Lance": { "boosters": 6, "shields": [ 3, 5 ] },
    "Hauler": { "boosters": 2, "shields": [ 1, 3 ] },
    "Imperial Clipper": { "boosters": 4, "shields": [ 3, 7 ] },
    "Imperial Courier": { "boosters": 4, "shields": [ 1, 3 ] },
    "Imperial Cutter": { "boosters": 8, "shields": [ 6, 8 ] },
    "Imperial Eagle": { "boosters": 1, "shields": [ 1, 3 ] },
    "Keelback": { "boosters": 3, "shields": [ 3, 5 ] },
    "Krait Mk II": { "boosters": 4, "shields": [ 3, 6 ] },
    "Krait Phantom": { "boosters": 4, "shields": [ 3, 6 ] },
    "Mamba": { "boosters": 6, "shields": [ 3, 5 ] },
    "Orca": { "boosters": 4, "shields": [ 3, 6 ] },
    "Python": { "boosters": 4, "shields": [ 3, 6 ] },
    "Sidewinder": { "boosters": 2, "shields": [ 1, 2 ] },
    "Type-10 Defender": { "boosters": 8, "shields": [ 6, 8 ] },
    "Type-6 Transporter": { "boosters": 3, "shields": [ 3, 5 ] },
    "Type-7 Transporter": { "boosters": 4, "shields": [ 3, 6 ] },
    "Type-9 Heavy": { "boosters": 4, "shields": [ 5, 8 ] },
    "Viper": { "boosters": 2, "shields": [ 1, 3 ] },
    "Viper Mk IV": { "boosters": 2, "shields": [ 3, 4 ] },
    "Vulture": { "boosters": 4, "shields": [ 3, 5 ] }
  };
  const Boosters = ['A', 'B', 'C', 'D', 'E'];
  const Defaults = {
    e: 0,
    t: 0,
    k: 0,
    a: 0,
    effectiveness: 50,
    ship: "Anaconda",
    shield_class: 7,
    boosters: 2,
    prismatic: false,
    reinforced_mj: 0,
    scb_mj: 0,
    regen_time_limit: 0,
    booster_rating: Boosters[0]
  };

  let form = JSON.parse(JSON.stringify(Defaults));

  route();
  let Reset = JSON.parse(JSON.stringify(form));
  let Requesting = false;
  let Results = false;

  function route() {
    if (typeof window !== 'undefined' && window.location && window.location.href) {
      window.location.href.replace(/[?&]+([^=&]+)=([^&]*)/gi, function(m,key,value) {
        if (typeof form[key] == 'number') {
          if (/^\d+$/.test(value)) {
            form[key] = Number(value);
          }
        } else {
          if (key == "prismatic") {
            form[key] = value == "true";
          } else if (key != "ship" || Ships[value]) {
            form[key] = value;
          }
        }
      });
    }
  }

  function request(url, method) {
    var req = new XMLHttpRequest();
    return new Promise(function(resolve, reject) {
       req.onreadystatechange = function () {
        if (req.readyState !== 4) return;

        if (req.status >= 200 && req.status < 300) {
          resolve(req);
        } else {
          reject({
            status: req.status,
            statusText: req.statusText
          });
        }
      };

      req.open(method || 'GET', url, true);
      req.send();
    });
  }

  function formToQuery() {
    return Object.keys(form).map(key => key + '=' + form[key]).join('&');
  }

  function handleReset() {
    form = JSON.parse(JSON.stringify(Reset));
  }

  function handleReboot() {
    form = JSON.parse(JSON.stringify(Defaults));
    Results = false;
    history.pushState(form, "", BaseURL);
  }

  function handleSubmit(event) {
    if (Requesting) {
      return;
    }

    pushState();

    Requesting = true;
    request(BaseURL + 'calculate.php?' + formToQuery())
      .then(function(resp) {
        Results = resp.response;
        Requesting = false;
        setTimeout(function() {
          document.getElementById('Result').scrollIntoView();
        }, 100);
      })
      .catch(function(err) {
        Results = "Whoops, that didn't work: " + err.statusText;
        Requesting = false;
      });
  }

  function pushState() {
    history.pushState(form, "", BaseURL + "?" + formToQuery());
  }

  function handleBackNavigation(event) {
    route();
  }
</script>

<svelte:window on:popstate={handleBackNavigation} />

<form id="Testform" action="{BaseURL}calculate.php" method="get" on:submit|preventDefault={handleSubmit}>
  <fieldset><legend>Attacker Damage Per Second</legend>
    <label>Explosive<br>
      <Range
        name="e"
        min="0"
        max="200"
        bind:value={form.e}></Range>
    </label><br>

    <label>Thermal<br>
      <Range
        name="t"
        min="0"
        max="200"
        bind:value={form.t}></Range>
    </label><br>

    <label>Kinetic<br>
      <Range
        name="k"
        min="0"
        max="200"
        bind:value={form.k}></Range>
    </label><br>

    <label>Absolute<br>
      <Range
        name="a"
        min="0"
        max="200"
        bind:value={form.a}></Range>
    </label><br>

    <label class="effectiveness">Effectiveness (% of time taking damage)<br>
      <Range
        name="effectiveness"
        min="1"
        max="100"
        bind:value={form.effectiveness}></Range>
    </label>
  </fieldset>

  <fieldset><legend>Defender</legend>
    <label>Ship
      <select
        class="select-css"
        name="ship"
        bind:value={form.ship}>
        {#each Object.keys(Ships) as ship}
          <option value={ship}>{ship}</option>
        {/each}
      </select>
    </label><br>

    <label>Shield Class<br>
      <Range
        name="shield_class"
        min="{Ships[form.ship].shields[0]}"
        max="{Ships[form.ship].shields[1]}"
        bind:value={form.shield_class}></Range>
    </label><br>

    <label>Shield Boosters<br>
      <Range
        name="boosters"
        min="1"
        max="{Ships[form.ship].boosters}"
        bind:value={form.boosters}></Range>
    </label><br>

    <label>Shield Booster Rating<br>
      <select
        class="select-css"
        name="booster_rating"
        bind:value={form.booster_rating}>
        {#each Boosters as rating}
          <option value={rating}>{rating}</option>
        {/each}
      </select>
    </label><br>

    <label>Shield Cell Reinforcement (Mj)<br>
      <Range
        name="scb_mj"
        min="0"
        max="10000"
        bind:value={form.scb_mj}></Range>
    </label><br>

    <label>Guardian Shield Reinforcement (Mj)<br>
      <Range
        name="reinforced_mj"
        min="0"
        max="2000"
        bind:value={form.reinforced_mj}>
      </Range>
    </label><br>

    <label>Minimum Regen Time in Seconds (from 50%)<br>
      <Range
        name="regen_time_limit"
        min="0"
        max="1800"
        bind:value={form.regen_time_limit}></Range>
    </label><br>

    <label for="prismatic">Allow Prismatic Shields</label>
    <input type="checkbox" name="prismatic" bind:checked={form.prismatic}>
  </fieldset>

  <fieldset><legend>Neat Buttons</legend>
    <button type="submit" disabled={Requesting === true}>{#if Requesting === true}Calculating&hellip;{:else}Calculate{/if}</button>
    <button type="button" on:click={handleReset} title="Reset to this URL's parameters">Reset</button>
    <button type="button" on:click={handleReboot} title="Reset to default parameters">Restart</button>
  </fieldset>

  {#if Results !== false}
    <fieldset id="Result"><legend>Result</legend>
      <pre>{Results}</pre>
    </fieldset>
  {/if}
</form>
