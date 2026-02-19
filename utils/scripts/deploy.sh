#!/bin/bash


cd front/apps/website/dist; 
7z a "${buid_name}" ./;
mv "${buid_name}" ../../../back/target/release;

7z a "${buid_name}" ap-bak;

mv "${buid_name}" ../../../back/target/release;

	# @scp front/apps/website/dist/ap.7z eve-dev:/home/ubuntu/apps