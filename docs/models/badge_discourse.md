# Badge

## Description

frozen_string_literal: true

## Relationships

- belongs_to :badge_type
- belongs_to :badge_grouping
- belongs_to :image_upload
- has_many :user_badges
- has_many :upload_references

## Methods

### self

frozen_string_literal: true

class Badge < ActiveRecord::Base
  include GlobalPath
  include HasSanitizableFields

  # NOTE: These badge ids are not in order! They are grouped logically.
  #       When picking an id, *search* for it.

  BasicUser = 1
  Member = 2
  Regular = 3
  Leader = 4

  Welcome = 5
  NicePost = 6
  GoodPost = 7
  GreatPost = 8
  Autobiographer = 9
  Editor = 10
  WikiEditor = 48

  FirstLike = 11
  FirstShare = 12
  FirstFlag = 13
  FirstLink = 14
  FirstQuote = 15
  FirstMention = 40
  FirstEmoji = 41
  FirstOnebox = 42
  FirstReplyByEmail = 43

  ReadGuidelines = 16
  Reader = 17
  NiceTopic = 18
  GoodTopic = 19
  GreatTopic = 20
  NiceShare = 21
  GoodShare = 22
  GreatShare = 23
  Anniversary = 24

  Promoter = 25
  Campaigner = 26
  Champion = 27

  PopularLink = 28
  HotLink = 29
  FamousLink = 30

  Appreciated = 36
  Respected = 37
  Admired = 31

  OutOfLove = 33
  HigherLove = 34
  CrazyInLove = 35

  ThankYou = 38
  GivesBack = 32
  Empathetic = 39

  Enthusiast = 45
  Aficionado = 46
  Devotee = 47

  NewUserOfTheMonth = 44

  # other consts
  AutobiographerMinBioLength = 10

  # used by serializer
  attr_accessor :has_badge

### self

fields that can not be edited on system badges

### update_user_titles

#
  # Update all user titles based on a badge to the new name

### reset_user_titles

#
  # When a badge has its TranslationOverride cleared, reset
  # all user titles granted to the standard name.

### display_name

allow to correct orphans
    if !self.badge_grouping_id || self.badge_grouping_id <= BadgeGrouping::Other
      self.badge_grouping_id = val
    end
  end

