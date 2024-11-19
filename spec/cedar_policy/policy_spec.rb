# frozen_string_literal: true

RSpec.describe CedarPolicy::Policy do
  subject { CedarPolicy::Policy.new }

  describe "with policy string" do
    context "when policy has id annotation" do
      let(:policy_str) do
        <<~POLICY
          @description("Happy user can see blue sky")
          permit (
            principal == User::"happy",
            action,
            resource == Photo::"blue_sky"
          );
        POLICY
      end


      describe "policy text is accessible" do
        subject { CedarPolicy::Policy.new(policy_str) }
        it { is_expected.to have_attributes(to_s: policy_str) }
      end

      describe "policy id is set when given" do
        subject { CedarPolicy::Policy.new(policy_str, id: "MyPolicyId") }
        it { is_expected.to have_attributes(id: "MyPolicyId") }
      end

      describe "policy annotations are accessible" do
        subject { CedarPolicy::PolicySet.new(policy_str, id_annotation: :description).policies.first }
        it { is_expected.to have_attributes(annotations: { "description" => "Happy user can see blue sky"}) }
      end
    end
  end
end
