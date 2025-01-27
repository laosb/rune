import 'package:fluent_ui/fluent_ui.dart';
import 'package:material_symbols_icons/symbols.dart';

import '../../../utils/l10n.dart';
import '../../../widgets/settings/settings_box_base.dart';

class EnableBroadcastSetting extends StatefulWidget {
  const EnableBroadcastSetting({
    super.key,
  });

  @override
  State<EnableBroadcastSetting> createState() => _EnableBroadcastSettingState();
}

class _EnableBroadcastSettingState extends State<EnableBroadcastSetting> {
  final _menuController = FlyoutController();

  Widget buildExpanderContent(BuildContext context) {
    final s = S.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Button(
          onPressed: () => {},
          child: Text(s.start),
        ),
      ],
    );
  }

  Widget buildDefaultContent(BuildContext context) {
    final s = S.of(context);

    return FlyoutTarget(
      controller: _menuController,
      child: Button(
        onPressed: () => {},
        child: Text(s.start),
      ),
    );
  }

  @override
  dispose() {
    _menuController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final s = S.of(context);

    return SettingsBoxBase(
      title: s.enableBroadcast,
      subtitle: s.enableBroadcastSubtitle,
      icon: Symbols.graph_5,
      buildExpanderContent: buildExpanderContent,
      buildDefaultContent: buildDefaultContent,
    );
  }
}
