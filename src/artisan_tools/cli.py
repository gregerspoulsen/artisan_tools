from artisan_tools.app import App


def run():
    """
    Run the CLI
    """
    app = App()
    app.load_extensions()
    return app.cli()
