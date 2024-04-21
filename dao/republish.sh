#!/bin/bash

resim reset
resim new-account
resim new-simple-badge

echo publish
resim publish .

echo hoard_create
resim run ./manifests/resim/hoard_create.rtm

echo dao_create
resim run ./manifests/resim/dao_create.rtm

echo dao_add_proposal
resim run ./manifests/resim/dao_add_proposal.rtm

resim set-current-epoch 3

echo vote with nft
resim run ./manifests/resim/proposal_mint_nft_vote.rtm
