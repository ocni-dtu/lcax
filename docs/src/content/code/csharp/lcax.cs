namespace LCAx
{

    public partial class Lcax
    {
        public string ClassificationSystem { get; set; }
        public string Comment { get; set; }
        public string Description { get; set; }
        public Dictionary<string, Assembly> EmissionParts { get; set; }
        public string FormatVersion { get; set; }
        public string Id { get; set; }
        public ImpactCategoryKey[] ImpactCategories { get; set; }
        public string LciaMethod { get; set; }
        public LifeCycleStage[] LifeCycleStages { get; set; }
        public long? LifeSpan { get; set; }
        public string Location { get; set; }
        public Dictionary<string, string> MetaData { get; set; }
        public string Name { get; set; }
        public Dictionary<string, Dictionary<string, double>> Results { get; set; }
    }

    public partial class Assembly
    {
        public string Category { get; set; }
        public Classification[] Classification { get; set; }
        public string Comment { get; set; }
        public string Description { get; set; }
        public string Id { get; set; }
        public Dictionary<string, string> MetaData { get; set; }
        public string Name { get; set; }
        public Dictionary<string, EpdPart> Parts { get; set; }
        public double Quantity { get; set; }
        public Dictionary<string, Dictionary<string, double>> Results { get; set; }
        public Unit Unit { get; set; }
    }

    public partial class Classification
    {
        public string Code { get; set; }
        public string Name { get; set; }
        public string System { get; set; }
    }

    public partial class EpdPart
    {
        public EpdSource EpdSource { get; set; }
        public string Id { get; set; }
        public Dictionary<string, string> MetaData { get; set; }
        public string Name { get; set; }
        public double PartQuantity { get; set; }
        public Unit PartUnit { get; set; }
        public double ReferenceServiceLife { get; set; }
        public Transport Transport { get; set; }
    }

    public partial class EpdSource
    {
        public Epd Epd { get; set; }
        public ExternalEpd Externalepd { get; set; }
        public InternalEpd Internalepd { get; set; }
    }

    public partial class Epd
    {
        public ImpactCategory Adpe { get; set; }
        public ImpactCategory Adpf { get; set; }
        public ImpactCategory Ap { get; set; }
        public string Comment { get; set; }
        public Conversion[] Conversions { get; set; }
        public ImpactCategory Cru { get; set; }
        public Unit DeclaredUnit { get; set; }
        public ImpactCategory Eee { get; set; }
        public ImpactCategory Eet { get; set; }
        public ImpactCategory Ep { get; set; }
        public string FormatVersion { get; set; }
        public ImpactCategory Fw { get; set; }
        public ImpactCategory Gwp { get; set; }
        public ImpactCategory Hwd { get; set; }
        public string Id { get; set; }
        public string Location { get; set; }
        public ImpactCategory Mer { get; set; }
        public Dictionary<string, string> MetaData { get; set; }
        public ImpactCategory Mrf { get; set; }
        public string Name { get; set; }
        public ImpactCategory Nhwd { get; set; }
        public ImpactCategory Nrsf { get; set; }
        public ImpactCategory Odp { get; set; }
        public ImpactCategory Penre { get; set; }
        public ImpactCategory Penrm { get; set; }
        public ImpactCategory Penrt { get; set; }
        public ImpactCategory Pere { get; set; }
        public ImpactCategory Perm { get; set; }
        public ImpactCategory Pert { get; set; }
        public ImpactCategory Pocp { get; set; }
        public DateTimeOffset PublishedDate { get; set; }
        public long? ReferenceServiceLife { get; set; }
        public ImpactCategory Rsf { get; set; }
        public ImpactCategory Rwd { get; set; }
        public ImpactCategory Sm { get; set; }
        public Source Source { get; set; }
        public Standard Standard { get; set; }
        public SubType Subtype { get; set; }
        public DateTimeOffset ValidUntil { get; set; }
        public string Version { get; set; }
    }

    public partial class ImpactCategory
    {
        public double? A1A3 { get; set; }
        public double? A4 { get; set; }
        public double? A5 { get; set; }
        public double? B1 { get; set; }
        public double? B2 { get; set; }
        public double? B3 { get; set; }
        public double? B4 { get; set; }
        public double? B5 { get; set; }
        public double? B6 { get; set; }
        public double? B7 { get; set; }
        public double? C1 { get; set; }
        public double? C2 { get; set; }
        public double? C3 { get; set; }
        public double? C4 { get; set; }
        public double? D { get; set; }
    }

    public partial class Conversion
    {
        public Unit To { get; set; }
        public double Value { get; set; }
    }

    public partial class Source
    {
        public string Name { get; set; }
        public string Url { get; set; }
    }

    public partial class ExternalEpd
    {
        public string Format { get; set; }
        public string Url { get; set; }
        public string Version { get; set; }
    }

    public partial class InternalEpd
    {
        public string Path { get; set; }
    }

    public partial class Transport
    {
        public double Distance { get; set; }
        public DistanceUnit DistanceUnit { get; set; }
        public string Id { get; set; }
        public string Name { get; set; }
        public TransportType TransportType { get; set; }
    }

    public enum Unit { Kg, L, M, M2, M2R1, M3, Pcs, Tones, Unknown };

    public enum Standard { En15804A1, En15804A2, Unknown };

    public enum SubType { Generic, Industry, Representative, Specific };

    public enum DistanceUnit { Km, M };

    public enum TransportType { Plane, Ship, Train, Truck };

    public enum ImpactCategoryKey { Adpe, Adpf, Ap, Cru, Eee, Eet, Ep, Fw, Gwp, Hwd, Mer, Mrf, Nhwd, Nrsf, Odp, Penre, Penrm, Penrt, Pere, Perm, Pert, Pocp, Rsf, Rwd, Sm };

    public enum LifeCycleStage { A1A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, C1, C2, C3, C4, D };
}
