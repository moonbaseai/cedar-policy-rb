# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class Entities
    include Enumerable

    # This is a hack to work around the current pathway for building the rust Entities object
    # in TryConvert, where we can't easily pass Schema to the Entities::from_json_value fn.
    attr_accessor :schema

    def initialize(entities = [])
      @entities = Set.new(entities.map do |entity|
        next entity if entity.is_a?(Entity)

        Entity.new(*entity.values_at(:uid, :attrs, :parents))
      end)
    end

    def each(&block)
      return enum_for(:each) unless block_given?

      @entities.each(&block)
    end

    def to_ary
      @entities.map { |entity| CedarPolicy.deep_serialize(entity) }
    end
  end
end
