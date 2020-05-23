# atrofac

A library and a command line application to control the power plan, and the fan curve (CPU & GPU) of Asus Zephyrus G14 devices (might also work with other devices that use the Armoury Crate Service). Fanless mode is possible as long as the GPU & CPU temperatures are not too hot (even on battery). 

## WARING / DISCLAIMER

**USE THIS AT YOUR OWN RISK. MANUALLY ADJUSTING FAN CURVES CAN BE DANGEROUS (MIGHT BREAK YOUR DEVICE). FRANKLY I HAVE NO IDEA WHAT I'M DOING SINCE THERE'S NO DOCUMENTATION FROM ASUS ON HOW TO MANUALLY ADJUST THE FAN CURVES. ATROFAC TRIES TO PREVENT YOU FROM SETTING A DANGEROUS FAN CURVE (THE SAME WAY ARMOURY CRATE DOES).**

## State

This is a very early proof of concept:

 * Needs to be tested on other devices than just mine (firmware `GA401IU.212`, Ryzen 7 4800HS, GeForce GTX 1660 Ti).

## Getting started (end user documentation)

You most likely want to use the atrofac GUI (if not, see here for the command line version: [ADVANCED.md](ADVANCED.md)). It's a simple system tray application that runs in the background.

![System tray](bin/systray.png)

### Step 1: Download

Download the binary: [atrofac-gui.exe](bin/atrofac-gui.exe).

### Step 2: Autostart

You usually want atrofac to be started when your computer starts. You can also skip this step and start the application manually.

Press Windows-Key + R. This opens "Run":

![Run](bin/startup.png)

Enter `shell:startup`. Now move the "autrofac-gui" to the Start-up folder. Done.

![Run](bin/startup_folder.png)

### Step 3: Start the app

Start the app and you should see a new icon in the system tray; the app is running.

![Screenshot](bin/tray.png)

### More

atrofac comes preconfigured with 6 different profiles ("Silent (Fanless)", "Silent (Low-Speed-Fan)", ...). If you want to see what those profiles do - or change them, click on "Edit configuration"; the configuration is just a yaml file. After saving the file you have to click "Reload configuration" to apply the changes. If you break the configuration file, the app won't start up anymore. Don't worry: Just delete the broken configuration file and atrofac will create a new one. The file can be found here (note: the folder `AppData` is hidden by default):

``
C:\Users\<YOUR_USER_NAME>\AppData\Roaming\atrofac_gui_config.yaml
``  

An entry for a plan looks like this:

```yaml
  - name: Silent (Fanless)
    plan: silent
    update_interval_sec: 30
    cpu_curve: "30c:0%,40c:0%,50c:0%,60c:0%,70c:31%,80c:49%,90c:56%,100c:56%"
    gpu_curve: "30c:0%,40c:0%,50c:0%,60c:0%,70c:34%,80c:51%,90c:61%,100c:61%"
```

 * `name`: The name, obviously.
 * `plan`: One of `silent`, `windows`, `performance`, `turbo`.
 * `update_interval_sec`: Armoury crate will overwrite the changes made by atrofac eventually (usually when waking up from sleep; when going from AC to DC or vice versa). So atrofac will periodically apply the settings every n seconds. I suggest choosing a value between 5-240 seconds.
 * `cpu_curve`: CPU fan curve, see [ADVANCED.md](ADVANCED.md).
 * `gpu_curve`: GPU fan curve, see [ADVANCED.md](ADVANCED.md).
 
You can also omit `cpu_curve` and `gpu_curve` in that case the default fan curves (as defined by Asus) are used. 

## Advanced

See [ADVANCED.md](ADVANCED.md) if you need more information such as:

 * The command line version of this tool.
 * Building.
 * Technical information.
 * Fan-curve details.
 
## Binaries

There's no CI yet, but there are prebuilt binaries:

 * GUI: [atrofac-gui.exe](bin/atrofac-gui.exe)
 * CLI-tool: [atrofac-cli.exe](bin/atrofac-cli.exe)

