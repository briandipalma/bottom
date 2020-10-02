use clap::*;

const TEMPLATE: &str = "\
{bin} {version}
{author}

{about}

USAGE:{usage}

FLAGS:
{unified}";

const USAGE: &str = "
    btm [FLAG]";

pub fn get_matches() -> clap::ArgMatches<'static> {
    build_app().get_matches()
}

pub fn build_app() -> App<'static, 'static> {
    // Temps
    let kelvin = Arg::with_name("kelvin")
        .short("k")
        .long("kelvin")
        .help("Sets the temperature type to Kelvin.")
        .long_help(
            "\
Sets the temperature type to Kelvin.\n\n",
        );
    let fahrenheit = Arg::with_name("fahrenheit")
        .short("f")
        .long("fahrenheit")
        .help("Sets the temperature type to Fahrenheit.")
        .long_help(
            "\
Sets the temperature type to Fahrenheit.\n\n",
        );
    let celsius = Arg::with_name("celsius")
        .short("c")
        .long("celsius")
        .help("Sets the temperature type to Celsius.")
        .long_help(
            "\
Sets the temperature type to Celsius.  This is the default
option.\n\n",
        );

    // All flags.  These are in alphabetical order
    let autohide_time = Arg::with_name("autohide_time")
        .long("autohide_time")
        .help("Temporarily shows the time scale in graphs.")
        .long_help(
            "\
Automatically hides the time scaling in graphs after being
shown for a brief moment when zoomed in/out.  If time is
disabled via --hide_time then this will have no effect.\n\n\n",
        );
    let basic = Arg::with_name("basic")
        .short("b")
        .long("basic")
        .help("Hides graphs and uses a more basic look.")
        .long_help(
            "\
Hides graphs and uses a more basic look.  Design is largely
inspired by htop's.\n\n",
        );
    let battery = Arg::with_name("battery")
        .long("battery")
        .help("Shows the battery widget.")
        .long_help(
            "\
Shows the battery widget in default or basic mode. No effect on
custom layouts.\n\n",
        );
    let case_sensitive = Arg::with_name("case_sensitive")
        .short("S")
        .long("case_sensitive")
        .help("Enables case sensitivity by default.")
        .long_help(
            "\
When searching for a process, enables case sensitivity by default.\n\n",
        );
    let debug = Arg::with_name("debug")
        .long("debug")
        .help("Enables debug logging.")
        .long_help(
            "\
Enables debug logging.  The program will print where it logged to after running.",
        );
    let disable_click = Arg::with_name("disable_click")
        .long("disable_click")
        .help("Disables mouse clicks.")
        .long_help(
            "\
Disables mouse clicks from interacting with the program.\n\n",
        );
    let dot_marker = Arg::with_name("dot_marker")
        .short("m")
        .long("dot_marker")
        .help("Uses a dot marker for graphs.")
        .long_help(
            "\
Uses a dot marker for graphs as opposed to the default braille
marker.\n\n",
        );
    let group = Arg::with_name("group")
        .short("g")
        .long("group")
        .help("Groups processes with the same name by default.")
        .long_help(
            "\
Groups processes with the same name by default.\n\n",
        );
    let hide_avg_cpu = Arg::with_name("hide_avg_cpu")
        .short("a")
        .long("hide_avg_cpu")
        .help("Hides the average CPU usage.")
        .long_help(
            "\
Hides the average CPU usage from being shown.\n\n",
        );
    let hide_table_gap = Arg::with_name("hide_table_gap")
        .long("hide_table_gap")
        .help("Hides the spacing between table headers and entries.")
        .long_help(
            "\
Hides the spacing between table headers and entries.\n\n",
        );
    let hide_time = Arg::with_name("hide_time")
        .long("hide_time")
        .help("Completely hides the time scaling.")
        .long_help(
            "\
Completely hides the time scaling from being shown.\n\n",
        );
    let left_legend = Arg::with_name("left_legend")
        .short("l")
        .long("left_legend")
        .help("Puts the CPU chart legend to the left side.")
        .long_help(
            "\
Puts the CPU chart legend to the left side rather than the right side.\n\n",
        );
    let no_write = Arg::with_name("no_write")
        .long("no_write")
        .help("Disables writing to the config file.")
        .long_help(
            "\
Disables config changes in-app from writing to the config file.",
        );
    let regex = Arg::with_name("regex")
        .short("R")
        .long("regex")
        .help("Enables regex by default.")
        .long_help(
            "\
When searching for a process, enables regex by default.\n\n",
        );
    let current_usage = Arg::with_name("current_usage")
        .short("u")
        .long("current_usage")
        .help("Sets process CPU% to be based on current CPU%.")
        .long_help(
            "\
Sets process CPU% usage to be based on the current system CPU% usage
rather than total CPU usage.\n\n",
        );
    let use_old_network_legend = Arg::with_name("use_old_network_legend")
        .long("use_old_network_legend")
        .help("DEPRECATED - uses the older network legend.")
        .long_help(
            "\
DEPRECATED - uses the older (pre-0.4) network widget legend.
This display is not tested anymore and could be broken.\n\n\n",
        );
    let whole_word = Arg::with_name("whole_word")
        .short("W")
        .long("whole_word")
        .help("Enables whole-word matching by default.")
        .long_help(
            "\
When searching for a process, return results that match the
entire query by default.\n\n",
        );

    // All options.  Again, alphabetical order.
    let config_location = Arg::with_name("config_location")
        .short("C")
        .long("config")
        .takes_value(true)
        .value_name("CONFIG PATH")
        .help("Sets the location of the config file.")
        .long_help(
            "\
Sets the location of the config file.  Expects a config
file in the TOML format. If it doesn't exist, one is created.\n\n\n",
        );
    let default_time_value = Arg::with_name("default_time_value")
        .short("t")
        .long("default_time_value")
        .takes_value(true)
        .value_name("MS")
        .help("Default time value for graphs in ms.")
        .long_help(
            "\
Default time value for graphs in milliseconds.  The minimum
time is 30s (30000), and the default is 60s (60000).\n\n\n",
        );
    let default_widget_count = Arg::with_name("default_widget_count")
        .long("default_widget_count")
        .takes_value(true)
        .requires_all(&["default_widget_type"])
        .value_name("INT")
        .help("Sets the n'th selected widget type as the default.")
        .long_help(
            "\
Sets the n'th selected widget type to use as the default widget.
Requires 'default_widget_type' to also be set, and defaults to 1.

This reads from left to right, top to bottom.  For example, suppose
we have a layout that looks like:
+-------------------+-----------------------+
|      CPU (1)      |        CPU (2)        |
+---------+---------+-------------+---------+
| Process | CPU (3) | Temperature | CPU (4) |
+---------+---------+-------------+---------+

And we set our default widget type to 'CPU'.  If we set
'--default_widget_count 1', then it would use the CPU (1) as
the default widget.  If we set '--default_widget_count 3', it would
use CPU (3) as the default instead.
\n\n",
        );
    let default_widget_type = Arg::with_name("default_widget_type")
        .long("default_widget_type")
        .takes_value(true)
        .value_name("WIDGET TYPE")
        .help("Sets which widget type to use as the default widget.")
        .long_help(
            "\
Sets which widget type to use as the default widget.
For the default layout, this defaults to the 'process' widget.
For a custom layout, it defaults to the first widget it sees.

For example, suppose we have a layout that looks like:
+-------------------+-----------------------+
|      CPU (1)      |        CPU (2)        |
+---------+---------+-------------+---------+
| Process | CPU (3) | Temperature | CPU (4) |
+---------+---------+-------------+---------+

Setting '--default_widget_type Temp' will make the Temperature
widget selected by default.

Supported widget names:
+--------------------------+
|            cpu           |
+--------------------------+
|        mem, memory       |
+--------------------------+
|       net, network       |
+--------------------------+
| proc, process, processes |
+--------------------------+
|     temp, temperature    |
+--------------------------+
|           disk           |
+--------------------------+
|       batt, battery      |
+--------------------------+
\n\n",
        );
    let rate = Arg::with_name("rate")
        .short("r")
        .long("rate")
        .takes_value(true)
        .value_name("MS")
        .help("Sets a refresh rate in ms.")
        .long_help(
            "\
Sets a refresh rate in milliseconds.  The minimum is 250ms,
and defaults to 1000ms.  Smaller values may take more resources.\n\n\n",
        );
    let time_delta = Arg::with_name("time_delta")
        .short("d")
        .long("time_delta")
        .takes_value(true)
        .value_name("MS")
        .help("The amount in ms changed upon zooming.")
        .long_help(
            "\
The amount of time in milliseconds changed when zooming in/out.
The minimum is 1s (1000), and defaults to 15s (15000).\n\n\n",
        );

    App::new(crate_name!())
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .template(TEMPLATE)
        .usage(USAGE)
        .help_message("Prints help information.  Use --help for more info.")
        .version_message("Prints version information.")
        .arg(kelvin)
        .arg(fahrenheit)
        .arg(celsius)
        .group(ArgGroup::with_name("TEMPERATURE_TYPE").args(&["kelvin", "fahrenheit", "celsius"]))
        .arg(autohide_time)
        .arg(basic)
        .arg(battery)
        .arg(case_sensitive)
        .arg(config_location)
        .arg(debug)
        .arg(default_time_value)
        .arg(default_widget_count)
        .arg(default_widget_type)
        .arg(disable_click)
        .arg(dot_marker)
        .arg(group)
        .arg(hide_avg_cpu)
        .arg(hide_table_gap)
        .arg(hide_time)
        .arg(left_legend)
        .arg(no_write)
        .arg(rate)
        .arg(regex)
        .arg(time_delta)
        .arg(current_usage)
        .arg(use_old_network_legend)
        .arg(whole_word)
}