# PostActionType

## Description

frozen_string_literal: true

## Methods

### expire_cache

frozen_string_literal: true

class PostActionType < ActiveRecord::Base
  POST_ACTION_TYPE_ALL_FLAGS_KEY = "post_action_type_all_flags"
  POST_ACTION_TYPE_PUBLIC_TYPE_IDS_KEY = "post_action_public_type_ids"
  LIKE_POST_ACTION_ID = 2

  after_save { expire_cache if !skip_expire_cache_callback }
  after_destroy { expire_cache if !skip_expire_cache_callback }

  attr_accessor :skip_expire_cache_callback

  include AnonCacheInvalidator

