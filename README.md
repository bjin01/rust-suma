# rust-suma
This is my first rust learning app that utilizes SUSE Manager xmlrpc API to patch systems.

The program reads a yaml config file which holds all required login and system information.

You can patch systems with all or selected errata type defined in the yaml file.

## Benefits:
* This command line based tool can run from any linux host. 
* This binary does not require any further library as it is compiled unless like python scripts.
* The program is written in rust and bleeding fast.
* The program uses a given yaml file with api login so that only root user should have read permissions, no need to supply password into command line.
* Feel free to use it with crontab or in conjunction with 3rd party automation tools.

## Usage:
Download the compiled binary to a linux host:

```
cd /usr/local/bin
wget https://github.com/bjin01/rust-suma/raw/master/bin/uysupatch
chmod +x /usr/local/bin/uysupatch
```
Create your config file in yaml format:
```
# cat test.yaml 
hostname: yoursuma-server.bo2go.home
user_name: admin
password: jklöadfiuw
advisory_type: 
output_fields:
  - id
  - advisory_synopsis
  - advisory_type
  - date
servers: 
  - caasp01.bo2go.home
  - caasp02.bo2go.home
```
Finally execute the cli:
```# uysupatch -c ./test.yaml``

## Sample output:
```
# uysupatch -c ./test.yaml 
suma host api url: "http://bjsuma.bo2go.home/rpc/api"
advisory_synopsis: Recommended update for c-ares
advisory_type: Bug Fix Advisory
date: 9/13/21
id: 34327

advisory_synopsis: Recommended update for cronie
advisory_type: Bug Fix Advisory
date: 9/9/21
id: 34295
...
...
Patch Job ID 11812

Logout successful - 1
```

## Yaml Parameters:
You can define desired advisory type with one of:  
'Security Advisory', 'Product Enhancement Advisory', 'Bug Fix Advisory'

If you leave "advisory_type:" empty then all patches will be applied.

```advisory_type: Security Advisory```

Under ```servers:``` you simply list all systems which should be patched.

Under ```output_fields:``` you could provide the field names you want to see in stdout.

Once the command finished running successfully you could find the jobs under "schedules" in SUSE Manager web UI.

  
