<?php

/* This garbage fire brought to you by laziness and regret */

define('TESTER_PATH', "/home/freaky/elite_shield_tester/");
define('LOCK_PATH', '/tmp/shield_tester.lock');
define('OWN_URL', '/shieldtester/');

$Ships = array(
  'Adder' => array('boosters' => 2, 'shields' => array(1,3)),
  'Alliance Challenger' => array('boosters' => 4, 'shields' => array(4,6)),
  'Alliance Chieftain' => array('boosters' => 4, 'shields' => array(3,6)),
  'Alliance Crusader' => array('boosters' => 4, 'shields' => array(4,6)),
  'Anaconda' => array('boosters' => 8, 'shields' => array(3,7)),
  'Asp Explorer' => array('boosters' => 4, 'shields' => array(3,6)),
  'Asp Scout' => array('boosters' => 2, 'shields' => array(3,5)),
  'Beluga Liner' => array('boosters' => 6, 'shields' => array(5,6)),
  'Cobra Mk III' => array('boosters' => 2, 'shields' => array(3,4)),
  'Cobra Mk IV' => array('boosters' => 2, 'shields' => array(3,4)),
  'Diamondback Explorer' => array('boosters' => 4, 'shields' => array(3,4)),
  'Diamondback Scout' => array('boosters' => 4, 'shields' => array(3,3)),
  'Dolphin' => array('boosters' => 3, 'shields' => array(3,5)),
  'Eagle' => array('boosters' => 1, 'shields' => array(1,3)),
  'Federal Assault Ship' => array('boosters' => 4, 'shields' => array(4,5)),
  'Federal Corvette' => array('boosters' => 8, 'shields' => array(5,7)),
  'Federal Dropship' => array('boosters' => 4, 'shields' => array(4,6)),
  'Federal Gunship' => array('boosters' => 4, 'shields' => array(4,6)),
  'Fer-de-Lance' => array('boosters' => 6, 'shields' => array(3,5)),
  'Hauler' => array('boosters' => 2, 'shields' => array(1,3)),
  'Imperial Clipper' => array('boosters' => 4, 'shields' => array(3,7)),
  'Imperial Courier' => array('boosters' => 4, 'shields' => array(1,3)),
  'Imperial Cutter' => array('boosters' => 8, 'shields' => array(6,8)),
  'Imperial Eagle' => array('boosters' => 1, 'shields' => array(1,3)),
  'Keelback' => array('boosters' => 3, 'shields' => array(3,5)),
  'Krait Mk II' => array('boosters' => 4, 'shields' => array(3,6)),
  'Krait Phantom' => array('boosters' => 4, 'shields' => array(3,6)),
  'Mamba' => array('boosters' => 6, 'shields' => array(3,5)),
  'Orca' => array('boosters' => 4, 'shields' => array(3,6)),
  'Python' => array('boosters' => 4, 'shields' => array(3,6)),
  'Sidewinder' => array('boosters' => 2, 'shields' => array(1,2)),
  'Type-10 Defender' => array('boosters' => 8, 'shields' => array(6,8)),
  'Type-6 Transporter' => array('boosters' => 3, 'shields' => array(3,5)),
  'Type-7 Transporter' => array('boosters' => 4, 'shields' => array(3,6)),
  'Type-9 Heavy' => array('boosters' => 4, 'shields' => array(5,8)),
  'Viper' => array('boosters' => 2, 'shields' => array(1,3)),
  'Viper Mk IV' => array('boosters' => 2, 'shields' => array(3,4)),
  'Vulture' => array('boosters' => 4, 'shields' => array(3,5)),
);

function try_lock($kind = "") {
  $fp = @fopen(LOCK_PATH . $kind, "w");
  if ($fp && @flock($fp, LOCK_EX)) {
    return $fp;
  }
}

function release_lock($fp) {
  @flock($fp, LOCK_UN);
  @fclose($fp);
}

function try_calculate() {
  global $Ships;

  $path = TESTER_PATH;
  $flags = array(
    't' => '-t',
    'k' => '-k',
    'e' => '-e',
    'a' => '-a',
    'effectiveness' => '-d',
    'scb_mj' => '--shield-cell-mj',
    'reinforced_mj' => '--reinforced-mj',
    'regen_time_limit' => '--regen-time-limit',
    'shield_class' => '--shield-class',
    'boosters' => '-s'
  );

  $args = array();
  foreach ($flags as $name => $flag) {
    if (isset($_GET[$name]) && is_numeric($_GET[$name])) {
      $val = abs((int)$_GET[$name]);
      if ($name == 'effectiveness') {
        $val /= 100.0;
      }

      if ($name == 'regen_time_limit' && $val == 0) {
        continue;
      }

      $args[] = "$flag " . $val;
    }
  }

  $trim = 0;
  $kind = ".fast";

  if (isset($_GET['ship']) && isset($Ships[$_GET['ship']])) {
    $args[] = "--ship '" . $_GET['ship'] . "'";
  }

  if (!empty($args) && !isset($_GET['prismatic']) || $_GET['prismatic'] != 'true') {
    $args[] = "--disable-prismatic";
  }

  $boosters = max(1, min((int)@$_GET['boosters'], 8));

  if ($boosters > 6) {
    // Quick-running boosters get one lock
    // The two slightly slower ones each get their own
    $kind = ".$boosters";
  }

  $result = "";

  if (!empty($args)) {
    if ($l = try_lock($kind)) {
      $args = implode(" ", $args);
      @exec("$path/elite_shield_tester $args", $result);
      $result = implode("\n", array_slice($result, $trim));
      $result = str_replace(TESTER_PATH, "", $result);
      release_lock($l);
    } else {
      $result = "Couldn't get a lock. Try later :(";
    }
  }

  return $result;
}

header("Content-Type: text/plain");
echo try_calculate();

?>
