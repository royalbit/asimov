# Forge Protocol - Documentation Build
#
# Targets for generating presentation PDFs from Marp markdown

.PHONY: help presentation presentation-pdf presentation-pptx lint clean

# Check if marp-cli is installed
HAS_MARP := $(shell command -v marp 2> /dev/null)

help:
	@echo "🔥 Forge Protocol - Available Commands"
	@echo ""
	@echo "Presentation:"
	@echo "  make presentation       - Generate PDF presentation"
	@echo "  make presentation-pdf   - Generate PDF presentation"
	@echo "  make presentation-pptx  - Generate PowerPoint presentation"
	@echo ""
	@echo "Quality:"
	@echo "  make lint               - Run markdownlint on all docs"
	@echo "  make lint-fix           - Auto-fix markdown issues"
	@echo ""
	@echo "Utilities:"
	@echo "  make install-tools      - Show installation commands"
	@echo "  make clean              - Remove generated files"

presentation: presentation-pdf
	@echo ""
	@echo "✅ Presentation generated: Forge_Protocol_Suite.pdf"

presentation-pdf:
	@echo "📊 Generating PDF presentation..."
ifndef HAS_MARP
	@echo "⚠️  Marp CLI not found. Installing..."
	@npm install -g @marp-team/marp-cli
endif
	@marp docs/PRESENTATION.md -o Forge_Protocol_Suite.pdf --pdf --allow-local-files
	@echo "✅ Generated: Forge_Protocol_Suite.pdf"
	@ls -lh Forge_Protocol_Suite.pdf

presentation-pptx:
	@echo "📊 Generating PowerPoint presentation..."
ifndef HAS_MARP
	@echo "⚠️  Marp CLI not found. Installing..."
	@npm install -g @marp-team/marp-cli
endif
	@marp docs/PRESENTATION.md -o Forge_Protocol_Suite.pptx --pptx --allow-local-files
	@echo "✅ Generated: Forge_Protocol_Suite.pptx"
	@ls -lh Forge_Protocol_Suite.pptx

lint:
	@echo "📝 Linting markdown files..."
	@npx markdownlint-cli2 '**/*.md'
	@echo "✅ Markdown lint passed"

lint-fix:
	@echo "🔧 Fixing markdown issues..."
	@npx markdownlint-cli2 '**/*.md' --fix
	@echo "✅ Markdown fixes applied"

install-tools:
	@echo "📦 Required tools:"
	@echo ""
	@echo "1. Marp CLI (presentation generation)"
	@echo "   npm install -g @marp-team/marp-cli"
	@echo ""
	@echo "2. markdownlint-cli2 (documentation validation)"
	@echo "   npm install -g markdownlint-cli2"
	@echo ""
	@echo "Current status:"
	@command -v marp >/dev/null 2>&1 && echo "  ✅ Marp CLI installed" || echo "  ❌ Marp CLI not found"
	@command -v npx >/dev/null 2>&1 && echo "  ✅ npx available" || echo "  ❌ npx not found"

clean:
	@echo "🧹 Cleaning generated files..."
	@rm -f Forge_Protocol_Suite.pdf Forge_Protocol_Suite.pptx
	@echo "✅ Clean complete"
