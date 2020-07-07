# 2020-07-07

 * Profile adjustments:
   * Profile *Silent (fanless)*: New values are `30c:0%,49c:0%,59c:0%,69c:0%,79c:31%,89c:49%,99c:56%,109c:56%` (cpu) / `30c:0%,49c:0%,59c:0%,69c:0%,79c:34%,89c:51%,99c:61%,109c:61%`. Note 30c vs. 39c!. **You need to manually adjust the configuration file or delete the configuration file (atrofac will then create a new configuration file with the new values).**
 * Logging: Timestamp & log fan curve.

# 2020-07-06
 
 * atrofac now re-applies the plan after wakeup (from sleep & hibernation); configurable (activated by default).
 * Profile adjustments:
   * Profile *Silent (fanless)*: New values are `39c:0%,49c:0%,59c:0%,69c:0%,79c:31%,89c:49%,99c:56%,109c:56%` (cpu) / `39c:0%,49c:0%,59c:0%,69c:0%,79c:34%,89c:51%,99c:61%,109c:61%`. (note 39c vs. 30c, 49c vs 40c, ...). Fanless mode is now possible up to 78 degrees (before: 69 degrees). **You need to manually adjust the configuration file or delete the configuration file (atrofac will then create a new configuration file with the new values).**
   * Profile *Silent (low-speed fan)*: New values are `30c:10%,49c:10%,59c:10%,69c:10%,79c:31%,89c:49%,99c:56%,109c:56%` (cpu) / `30c:0%,49c:0%,59c:0%,69c:0%,79c:34%,89c:51%,99c:61%,109c:61%` (gpu). Reason: A bit less fan speed fluctuation (still fluctuates a bit, especially within the first minute). **You need to manually adjust the configuration file or delete the configuration file (atrofac will then create a new configuration file with the new values).**
 * Added logging; configurable (activated by default).
 * Reload the configuration before setting a new plan (this prevents overwriting of changes made by the user when the user has forgotten to apply the new configuration).
 
# 2020-05-26

 * atrofac now no longer periodically re-applies the plan (due to increased power drain); configurable.