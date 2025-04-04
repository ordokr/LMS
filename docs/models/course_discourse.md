# DiscourseConnect

## Description

frozen_string_literal: true

## Methods

### self

frozen_string_literal: true

class DiscourseConnect < DiscourseConnectBase
  class BlankExternalId < StandardError
  end

  class BannedExternalId < StandardError
  end

### lookup_or_create_user_unsafe

we don't want to ban 0 from being an external id
    external_id = self.external_id.to_s

    raise BlankExternalId if external_id.blank?

    raise BannedExternalId, external_id if BANNED_EXTERNAL_IDS.include?(external_id.downcase)

    # we protect here to ensure there is no situation where the same external id
    # concurrently attempts to create or update sso records
    #
    # we can get duplicate HTTP requests quite easily (client rapid refresh) and this path does stuff such
    # as updating groups for a users and so on that can happen even after the sso record and user is there
    DistributedMutex.synchronize("sso_lookup_or_create_user_#{external_id}") do
      lookup_or_create_user_unsafe(ip_address)
    end
  end

  private

### synchronize_groups

ensure it's not staged anymore
    user.unstage!

    change_external_attributes_and_override(sso_record, user)

    if sso_record && (user = sso_record.user) && !user.active && !require_activation
      user.active = true
      user.save!
      user.enqueue_welcome_message("welcome_user") unless suppress_welcome_message
      user.set_automatic_groups
    end

    custom_fields.each { |k, v| user.custom_fields[k] = v }

    user.ip_address = ip_address

    user.admin = admin unless admin.nil?
    user.moderator = moderator unless moderator.nil?

    user.title = title unless title.nil?

    # optionally save the user and sso_record if they have changed
    user.user_avatar.save! if user.user_avatar
    user.save!

    user.set_automatic_groups if @email_changed && user.active

    # The user might require approval
    user.create_reviewable

    if bio && (user.user_profile.bio_raw.blank? || SiteSetting.discourse_connect_overrides_bio)
      user.user_profile.bio_raw = bio
      user.user_profile.save!
    end

    if website
      user.user_profile.website = website
      user.user_profile.save!
    end

    if location
      user.user_profile.location = location
      user.user_profile.save!
    end

    unless admin.nil? && moderator.nil?
      Group.refresh_automatic_groups!(:admins, :moderators, :staff)
    end

    sso_record.save!

    apply_group_rules(sso_record.user) if sso_record.user

    sso_record && sso_record.user
  end

### change_external_attributes_and_override

Use a mutex here to counter SSO requests that are sent at the same time with
    # the same email payload
    DistributedMutex.synchronize("discourse_single_sign_on_#{email}") do
      user = User.find_by_email(email) if !require_activation

      if !user
        user_params = {
          primary_email:
            UserEmail.new(email: email, primary: true) do |user_email|
              user_email.skip_normalize_email = true
            end,
          name: resolve_name,
          username: resolve_username,
          ip_address: ip_address,
          registration_ip_address: ip_address,
        }

        if SiteSetting.allow_user_locale && locale && LocaleSiteSetting.valid_value?(locale)
          user_params[:locale] = locale
        end

        user = User.new(user_params)

        if SiteSetting.must_approve_users && EmailValidator.can_auto_approve_user?(email)
          ReviewableUser.set_approved_fields!(user, Discourse.system_user)
        end

        begin
          user.save!
        rescue ActiveRecord::RecordInvalid => e
          if SiteSetting.verbose_discourse_connect_logging
            Rails.logger.error(
              "Verbose SSO log: User creation failed. External id: #{external_id}, New User (user_id: #{user.id}) Params: #{user_params} User Params: #{user.attributes} User Errors: #{user.errors.full_messages} Email: #{user.primary_email.attributes} Email Error: #{user.primary_email.errors.full_messages}",
            )
          end
          raise e
        end

        if SiteSetting.verbose_discourse_connect_logging
          Rails.logger.warn(
            "Verbose SSO log: New User (user_id: #{user.id}) Params: #{user_params} User Params: #{user.attributes} User Errors: #{user.errors.full_messages} Email: #{user.primary_email.attributes} Email Error: #{user.primary_email.errors.full_messages}",
          )
        end
      end

      if user
        if sso_record = user.single_sign_on_record
          sso_record.last_payload = unsigned_payload
          sso_record.external_id = external_id
        else
          if avatar_url.present?
            Jobs.enqueue(
              :download_avatar_from_url,
              url: avatar_url,
              user_id: user.id,
              override_gravatar: SiteSetting.discourse_connect_overrides_avatar,
            )
          end

          if profile_background_url.present?
            Jobs.enqueue(
              :download_profile_background_from_url,
              url: profile_background_url,
              user_id: user.id,
              is_card_background: false,
            )
          end

          if card_background_url.present?
            Jobs.enqueue(
              :download_profile_background_from_url,
              url: card_background_url,
              user_id: user.id,
              is_card_background: true,
            )
          end

          user.create_single_sign_on_record!(
            last_payload: unsigned_payload,
            external_id: external_id,
            external_username: username,
            external_email: email,
            external_name: name,
            external_avatar_url: avatar_url,
            external_profile_background_url: profile_background_url,
            external_card_background_url: card_background_url,
          )
        end
      end

      user
    end
  end

### resolve_username

change external attributes for sso record
    sso_record.external_username = username
    sso_record.external_email = email
    sso_record.external_name = name
    sso_record.external_avatar_url = avatar_url
    sso_record.external_profile_background_url = profile_background_url
    sso_record.external_card_background_url = card_background_url
  end

