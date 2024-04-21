#!/bin/bash

resim reset
resim new-account
resim new-simple-badge
resim publish .
resim run ./manifests/resim/hoard_create.rtm
resim run ./manifests/resim/dao_create.rtm