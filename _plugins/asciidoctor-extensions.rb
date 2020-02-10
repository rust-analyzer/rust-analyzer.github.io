require 'asciidoctor'
require 'asciidoctor/extensions'

class PrInlineMacro < Asciidoctor::Extensions::InlineMacroProcessor
  use_dsl

  named :pr

  def process parent, target, attrs
    pr = target
    target = "https://github.com/rust-analyzer/rust-analyzer/pull/#{pr}"
    parent.document.register :links, target
    (create_anchor parent, "<code>##{pr}</code>", type: :link, target: target).convert
  end
end

class CommitInlineMacro < Asciidoctor::Extensions::InlineMacroProcessor
  use_dsl

  named :commit

  def process parent, target, attrs
    hash = target
    target = "https://github.com/rust-analyzer/rust-analyzer/commit/#{hash}"
    parent.document.register :links, target
    (create_anchor parent, "<code>#{hash[0..6]}</code>", type: :link, target: target).convert
  end
end

class ReleaseInlineMacro < Asciidoctor::Extensions::InlineMacroProcessor
  use_dsl

  named :release

  def process parent, target, attrs
    date = target
    target = "https://github.com/rust-analyzer/rust-analyzer/releases/#{date}"
    parent.document.register :links, target
    (create_anchor parent, "<code>#{date}</code>", type: :link, target: target).convert
  end
end


Asciidoctor::Extensions.register do
  inline_macro PrInlineMacro
  inline_macro CommitInlineMacro
  inline_macro ReleaseInlineMacro
end
